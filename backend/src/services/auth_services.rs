use std::sync::Arc;

use super::{get_collection, Collections};
use crate::services::utils::jwt_utils as jwt;
use crate::services::utils::password_utils::{hash_password, verify_password};
use crate::{
    error::{AppError, AuthError},
    models::{
        account::{Account, Role},
        refresh_token::RefreshToken,
    },
    routes::auth::AuthPayload,
    AppState,
};
use axum::{extract::Request, http::header, middleware::Next, response::IntoResponse, Extension};
use mongodb::{bson::doc, Collection};

const REFRESH_TOKEN_EXPIRY_DURATION: chrono::TimeDelta = chrono::Duration::days(30);
const ACCESS_TOKEN_EXPIRY_DURATION: chrono::TimeDelta = chrono::Duration::minutes(15);

pub async fn create_super_user(state: &Arc<AppState>) -> Result<Account, AppError> {
    let accounts: Collection<Account> = get_collection(state, Collections::ACCOUNTS);
    accounts
        .find_one_and_delete(doc! { "username": "SuperUser" })
        .await?;

    let superuser = Account::new(
        "SuperUser",
        &hash_password(
            &state
                .env
                .superuser_password,
        ),
        &[Role::Admin, Role::User],
    );

    accounts
        .insert_one(superuser.clone())
        .await?;

    Ok(superuser)
}

pub async fn register(
    state: &Arc<AppState>,
    auth_payload: AuthPayload,
    roles: &[Role],
) -> Result<Account, AppError> {
    let accounts: Collection<Account> = get_collection(state, Collections::ACCOUNTS);
    if accounts
        .find_one(doc! { "username": &auth_payload.username })
        .await?
        .is_some()
    {
        return Err(AuthError::UsernameAlreadyTaken.into());
    }

    let new_account = Account::new(
        &auth_payload.username,
        &hash_password(&auth_payload.password),
        roles,
    );

    accounts
        .insert_one(new_account.clone())
        .await?;

    Ok(new_account)
}

pub async fn login(
    state: &Arc<AppState>,
    auth_payload: AuthPayload,
) -> Result<(String, String), AppError> {
    let accounts: Collection<Account> = get_collection(state, Collections::ACCOUNTS);
    let refresh_tokens: Collection<RefreshToken> =
        get_collection(state, Collections::REFRESH_TOKENS);

    let account = accounts
        .find_one(doc! { "username": &auth_payload.username })
        .await?
        .ok_or(AuthError::InvalidCredentials)?;

    if !verify_password(&account.hashed_password, &auth_payload.password) {
        return Err(AuthError::InvalidCredentials.into());
    }

    let access_token = jwt::generate_token(
        account.id,
        chrono::Utc::now()
            .checked_add_signed(ACCESS_TOKEN_EXPIRY_DURATION)
            .ok_or(AppError::Internal(anyhow::Error::msg(
                "Failed to create access token",
            )))?
            .timestamp(),
        &state
            .env
            .jwt_secret,
    )?;

    let refresh_token_expiry_timestamp = chrono::Utc::now()
        .checked_add_signed(REFRESH_TOKEN_EXPIRY_DURATION)
        .ok_or(AppError::Internal(anyhow::Error::msg(
            "Failed to create refresh token",
        )))?
        .timestamp();

    let refresh_token = RefreshToken::new(
        account.id,
        refresh_token_expiry_timestamp,
        &jwt::generate_token(
            account.id,
            refresh_token_expiry_timestamp,
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

pub async fn refresh(state: &Arc<AppState>, refresh_token: String) -> Result<String, AppError> {
    let refresh_tokens: Collection<RefreshToken> =
        get_collection(state, Collections::REFRESH_TOKENS);

    let retrieved_refresh_token = refresh_tokens
        .find_one(doc! { "token": &refresh_token })
        .await?
        .ok_or(AuthError::TokenInvalid)?;

    let claims = jwt::decode_token(
        &refresh_token,
        &state
            .env
            .jwt_secret,
    )?;

    if retrieved_refresh_token.expiry_timestamp < chrono::Utc::now().timestamp() {
        return Err(AuthError::TokenExpired.into());
    }

    let access_token = jwt::generate_token(
        claims.sub,
        chrono::Utc::now()
            .checked_add_signed(ACCESS_TOKEN_EXPIRY_DURATION)
            .ok_or(AppError::Internal(anyhow::Error::msg(
                "Failed to create access token",
            )))?
            .timestamp(),
        &state
            .env
            .jwt_secret,
    )?;

    Ok(access_token)
}

pub async fn logout(state: &Arc<AppState>, refresh_token: String) -> Result<String, AppError> {
    let refresh_tokens: Collection<RefreshToken> =
        get_collection(state, Collections::REFRESH_TOKENS);

    let deleted_count = refresh_tokens
        .delete_one(doc! { "token": &refresh_token })
        .await?
        .deleted_count;

    if deleted_count != 0 {
        Ok("Logged out".to_string())
    } else {
        Err(AuthError::Unauthorized.into())
    }
}

pub async fn revoke_all_refresh_tokens(
    state: &Arc<AppState>,
    username: String,
) -> Result<u64, AppError> {
    let accounts: Collection<Account> = get_collection(state, Collections::ACCOUNTS);
    let account = accounts
        .find_one(doc! { "username": &username })
        .await?
        .ok_or(AuthError::InvalidCredentials)?;

    if !verify_password(&account.hashed_password, &username) {
        return Err(AuthError::InvalidCredentials.into());
    }

    let refresh_tokens: Collection<RefreshToken> =
        get_collection(state, Collections::REFRESH_TOKENS);
    Ok(refresh_tokens
        .delete_many(doc! { "user_id": account.id })
        .await?
        .deleted_count)
}

pub async fn auth_guard(
    Extension(state): Extension<Arc<AppState>>,
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, AppError> {
    let accounts: Collection<Account> = get_collection(&state, Collections::ACCOUNTS);
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

    let account = accounts
        .find_one(doc! { "_id": claims.sub })
        .await?
        .ok_or(AuthError::InvalidCredentials)?;

    req.extensions_mut()
        .insert(account);
    Ok(next
        .run(req)
        .await)
}
