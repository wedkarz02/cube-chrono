use std::sync::Arc;

use super::{account_services, jwt_services};
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

const REFRESH_TOKEN_EXPIRATION: chrono::TimeDelta = chrono::Duration::days(30);
const ACCESS_TOKEN_EXPIRATION: chrono::TimeDelta = chrono::Duration::minutes(15);

// TODO (wedkarz): remove this and just use the "register" service function
pub async fn create_super_user(state: &Arc<AppState>) -> Result<Account, AppError> {
    if let Some(admin) = account_services::find_by_username(state, "SuperUser").await? {
        return Ok(admin);
    }

    let superuser = Account::new(
        "SuperUser",
        &hash_password(
            &state
                .env
                .superuser_password,
        ),
        &[Role::Admin, Role::User],
    );

    let account_id = superuser.id;
    account_services::insert(state, superuser).await?;
    account_services::find_by_id(state, account_id)
        .await?
        .ok_or(anyhow::Error::msg("New account not inserted").into())
}

pub async fn register(
    state: &Arc<AppState>,
    auth_payload: AuthPayload,
    roles: &[Role],
) -> Result<Account, AppError> {
    if account_services::find_by_username(state, &auth_payload.username)
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

    let account_id = new_account.id;
    account_services::insert(state, new_account).await?;
    account_services::find_by_id(state, account_id)
        .await?
        .ok_or(anyhow::Error::msg("New account not inserted").into())
}

pub async fn login(
    state: &Arc<AppState>,
    auth_payload: AuthPayload,
) -> Result<(String, String), AppError> {
    let account = account_services::find_by_username(state, &auth_payload.username)
        .await?
        .ok_or(AuthError::InvalidCredentials)?;

    if !verify_password(&account.hashed_password, &auth_payload.password) {
        return Err(AuthError::InvalidCredentials.into());
    }

    let access_token = jwt_services::generate_token(
        account.id,
        chrono::Utc::now()
            .checked_add_signed(ACCESS_TOKEN_EXPIRATION)
            .ok_or(anyhow::Error::msg("Failed to create access token"))?
            .timestamp(),
        &state
            .env
            .jwt_secret,
    )?;

    let refresh_token_expiry_timestamp = chrono::Utc::now()
        .checked_add_signed(REFRESH_TOKEN_EXPIRATION)
        .ok_or(anyhow::Error::msg("Failed to create refresh token"))?
        .timestamp();

    let refresh_token = RefreshToken::new(
        account.id,
        refresh_token_expiry_timestamp,
        &jwt_services::generate_token(
            account.id,
            refresh_token_expiry_timestamp,
            &state
                .env
                .jwt_secret,
        )?,
    );

    let refresh_out = refresh_token
        .token
        .clone();
    jwt_services::insert_refresh(state, refresh_token).await?;

    Ok((access_token, refresh_out))
}

pub async fn refresh(state: &Arc<AppState>, refresh_token: &str) -> Result<String, AppError> {
    if jwt_services::find_refresh_by_token(state, &refresh_token)
        .await?
        .is_none()
    {
        return Err(AuthError::TokenInvalid.into());
    }

    let claims = jwt_services::decode_token(
        &refresh_token,
        &state
            .env
            .jwt_secret,
    )?;

    let access_token = jwt_services::generate_token(
        claims.sub,
        chrono::Utc::now()
            .checked_add_signed(ACCESS_TOKEN_EXPIRATION)
            .ok_or(anyhow::Error::msg("Failed to create access token"))?
            .timestamp(),
        &state
            .env
            .jwt_secret,
    )?;

    Ok(access_token)
}

pub async fn logout(state: &Arc<AppState>, refresh_token: &str) -> Result<String, AppError> {
    let deleted_count = jwt_services::delete_refresh_by_token(state, refresh_token)
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
    username: &str,
) -> Result<u64, AppError> {
    let account = account_services::find_by_username(state, username)
        .await?
        .ok_or(AuthError::InvalidCredentials)?;

    // TODO: fix password verification
    if !verify_password(&account.hashed_password, &username) {
        return Err(AuthError::InvalidCredentials.into());
    }

    let deleted_count = jwt_services::delete_many_refresh_by_user_id(state, account.id)
        .await?
        .deleted_count;
    Ok(deleted_count)
}

pub async fn auth_guard(
    Extension(state): Extension<Arc<AppState>>,
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, AppError> {
    let access_token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| {
            v.to_str()
                .ok()
        })
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or(AuthError::Unauthorized)?;

    let claims = jwt_services::decode_token(
        access_token,
        &state
            .env
            .jwt_secret,
    )?;

    let account = account_services::find_by_id(&state, claims.sub)
        .await?
        .ok_or(AuthError::InvalidCredentials)?;

    req.extensions_mut()
        .insert(account);
    Ok(next
        .run(req)
        .await)
}
