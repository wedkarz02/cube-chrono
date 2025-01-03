use std::sync::Arc;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, PasswordVerifier, SaltString},
    Argon2, PasswordHash,
};
use axum::{
    extract::Request,
    http::{header, StatusCode},
    middleware::Next,
    response::IntoResponse,
    Extension, Json,
};
use axum_extra::json;
use mongodb::{
    bson::{doc, Uuid},
    Collection,
};
use serde::Deserialize;

use crate::{
    error::AuthError,
    models::{
        refresh_token::RefreshToken,
        user::{Role, User},
    },
    AppState,
};

use super::{get_collection, jwt};

#[derive(Deserialize)]
pub struct AuthPayload {
    username: String,
    password: String,
}

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
        .map_or(false, |res| res.is_ok())
}

pub async fn register(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<AuthPayload>,
) -> Result<impl IntoResponse, AuthError> {
    let users: Collection<User> = get_collection(&state, "users");

    if users
        .find_one(doc! { "username": &payload.username })
        .await
        .unwrap()
        .is_some()
    {
        return Err(AuthError::UserAlreadyExists);
    }

    let user = User {
        id: Uuid::new(),
        username: payload
            .username
            .clone(),
        hashed_password: hash_password(&payload.password),
        role: Role::User,
    };

    users
        .insert_one(user.clone())
        .await
        .unwrap();

    Ok((StatusCode::CREATED, Json(user)))
}

pub async fn login(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<AuthPayload>,
) -> Result<impl IntoResponse, AuthError> {
    let users: Collection<User> = get_collection(&state, "users");
    let refresh_tokens: Collection<RefreshToken> = get_collection(&state, "refresh_tokens");

    let user = users
        .find_one(doc! { "username": &payload.username })
        .await
        .unwrap()
        .ok_or(AuthError::InvalidCredentials)?;

    if !verify_password(&user.hashed_password, &payload.password) {
        return Err(AuthError::InvalidCredentials);
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
    );

    let refresh_token = RefreshToken {
        id: Uuid::new(),
        user_id: user.id,
        token: jwt::generate_token(
            user.id,
            chrono::Utc::now()
                .checked_add_signed(chrono::Duration::days(30))
                .unwrap()
                .timestamp() as usize,
            &state
                .env
                .jwt_secret,
        ),
    };

    refresh_tokens
        .insert_one(&refresh_token)
        .await
        .unwrap();

    Ok((
        StatusCode::OK,
        json!({ "access_token": access_token, "refresh_token": refresh_token.token }),
    ))
}

pub async fn logout(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<serde_json::Value>,
) -> Result<impl IntoResponse, AuthError> {
    let refresh_tokens: Collection<RefreshToken> = get_collection(&state, "refresh_tokens");
    let refresh_token = payload
        .get("refresh_token")
        .and_then(|v| v.as_str())
        .ok_or(AuthError::Unauthorized)?;

    let claims = jwt::decode_token(
        refresh_token,
        &state
            .env
            .jwt_secret,
    )?;

    let deleted_count = refresh_tokens
        .delete_one(doc! { "user_id": claims.sub })
        .await
        .unwrap()
        .deleted_count;

    Ok((
        StatusCode::OK,
        json!({ "message": if deleted_count != 0 { "Logged out" } else { "Not logged in" } }),
    ))
}

pub async fn refresh(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<serde_json::Value>,
) -> Result<impl IntoResponse, AuthError> {
    let refresh_tokens: Collection<RefreshToken> = get_collection(&state, "refresh_tokens");
    let refresh_token = payload
        .get("refresh_token")
        .and_then(|v| v.as_str())
        .ok_or(AuthError::Unauthorized)?;

    let claims = jwt::decode_token(
        refresh_token,
        &state
            .env
            .jwt_secret,
    )?;

    let stored_token = refresh_tokens
        .find_one(doc! { "user_id": claims.sub })
        .await
        .unwrap()
        .ok_or(AuthError::Unauthorized)?;

    if stored_token.token != refresh_token {
        return Err(AuthError::Unauthorized);
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
    );

    Ok((StatusCode::OK, json!({ "access_token": access_token })))
}

pub async fn auth_guard(
    Extension(state): Extension<Arc<AppState>>,
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, AuthError> {
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
        .await
        .unwrap()
        .ok_or(AuthError::InvalidCredentials)?;

    req.extensions_mut()
        .insert(user);
    Ok(next
        .run(req)
        .await)
}
