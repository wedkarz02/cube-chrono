use std::sync::Arc;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, PasswordVerifier, SaltString},
    Argon2, PasswordHash,
};
use axum::{extract::Request, http::header, middleware::Next, response::IntoResponse, Extension};
use mongodb::{bson::doc, Collection};

use crate::{
    error::{AppError, AuthError},
    models::{
        refresh_token::RefreshToken,
        user::{Role, User},
    },
    routes::auth::AuthPayload,
    AppState,
};

use super::{get_collection, jwt};

fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string()
}

fn verify_password(hash: &str, password: &str) -> bool {
    PasswordHash::new(hash)
        .map(|parsed_hash| Argon2::default().verify_password(password.as_bytes(), &parsed_hash))
        .is_ok_and(|res| res.is_ok())
}

pub async fn create_super_user(state: &Arc<AppState>) -> Result<User, AppError> {
    let users: Collection<User> = get_collection(state, "users");
    users
        .find_one_and_delete(doc! { "username": "SuperUser" })
        .await?;

    let superuser = User::from(
        "SuperUser",
        &hash_password(
            &state
                .env
                .superuser_password,
        ),
        &vec![Role::Admin, Role::User],
    );

    users
        .insert_one(superuser.clone())
        .await?;

    Ok(superuser)
}

pub async fn register(
    state: &Arc<AppState>,
    auth_payload: AuthPayload,
    role: Role,
) -> Result<User, AppError> {
    let users: Collection<User> = get_collection(&state, "users");
    if users
        .find_one(doc! { "username": &auth_payload.username })
        .await?
        .is_some()
    {
        return Err(AuthError::UserAlreadyExists.into());
    }

    let user = User::from(
        &auth_payload.username,
        &hash_password(&auth_payload.password),
        &vec![role],
    );

    users
        .insert_one(user.clone())
        .await?;

    Ok(user)
}

pub async fn login(
    state: &Arc<AppState>,
    auth_payload: AuthPayload,
) -> Result<(String, String), AppError> {
    let users: Collection<User> = get_collection(&state, "users");
    let refresh_tokens: Collection<RefreshToken> = get_collection(&state, "refresh_tokens");

    let user = users
        .find_one(doc! { "username": &auth_payload.username })
        .await?
        .ok_or(AuthError::InvalidCredentials)?;

    if !verify_password(&user.hashed_password, &auth_payload.password) {
        return Err(AuthError::InvalidCredentials.into());
    }

    let access_token = jwt::generate_token(
        user.id,
        chrono::Utc::now()
            .checked_add_signed(chrono::Duration::minutes(15))
            .unwrap()
            .timestamp() as usize,
        &state
            .env
            .jwt_secret,
    )?;

    let refresh_token = RefreshToken::from(
        user.id,
        &jwt::generate_token(
            user.id,
            chrono::Utc::now()
                .checked_add_signed(chrono::Duration::days(30))
                .unwrap()
                .timestamp() as usize,
            &state
                .env
                .jwt_secret,
        )?,
    );

    refresh_tokens
        .insert_one(&refresh_token)
        .await?;

    Ok((access_token, refresh_token.token))
}

pub async fn logout(state: &Arc<AppState>, tok: String) -> Result<String, AppError> {
    let refresh_tokens: Collection<RefreshToken> = get_collection(&state, "refresh_tokens");
    let claims = jwt::decode_token(
        &tok,
        &state
            .env
            .jwt_secret,
    )?;

    let deleted_count = refresh_tokens
        .delete_one(doc! { "user_id": claims.sub })
        .await?
        .deleted_count;

    Ok(if deleted_count != 0 {
        "Logged out"
    } else {
        "Not logged in"
    }
    .to_string())
}

pub async fn refresh(state: &Arc<AppState>, tok: String) -> Result<String, AppError> {
    tracing::debug!("got token: {}", tok);
    let refresh_tokens: Collection<RefreshToken> = get_collection(&state, "refresh_tokens");
    let claims = jwt::decode_token(
        &tok,
        &state
            .env
            .jwt_secret,
    )?;
    tracing::debug!("got claims: {:?}", claims);

    let stored_token = refresh_tokens
        .find_one(doc! { "user_id": claims.sub })
        .await?
        .ok_or(AuthError::Unauthorized)?;

    if stored_token.token != tok {
        return Err(AuthError::Unauthorized.into());
    }

    let access_token = jwt::generate_token(
        claims.sub,
        chrono::Utc::now()
            .checked_add_signed(chrono::Duration::minutes(15))
            .unwrap()
            .timestamp() as usize,
        &state
            .env
            .jwt_secret,
    )?;

    Ok(access_token)
}

pub async fn auth_guard(
    Extension(state): Extension<Arc<AppState>>,
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, AppError> {
    let users: Collection<User> = get_collection(&state, "users");
    let access_token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| {
            v.to_str()
                .ok()
        })
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or(AuthError::Unauthorized)?;

    let claims = jwt::decode_token(
        access_token,
        &state
            .env
            .jwt_secret,
    )?;

    let user = users
        .find_one(doc! { "_id": claims.sub })
        .await?
        .ok_or(AuthError::InvalidCredentials)?;

    req.extensions_mut()
        .insert(user);
    Ok(next
        .run(req)
        .await)
}
