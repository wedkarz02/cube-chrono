use std::sync::Arc;

use super::{account_services, jwt_services};
use crate::services::utils::password_utils::{hash_password, verify_password};
use crate::{
    error::{AppError, AuthError},
    models::account::{Account, Role},
    routes::auth::AuthPayload,
    AppState,
};
use axum::{extract::Request, http::header, middleware::Next, response::IntoResponse, Extension};
use mongodb::results::DeleteResult;

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

    let (access_token, refresh_token) = jwt_services::generate_pair(
        account.id,
        &state
            .env
            .jwt_access_secret,
        &state
            .env
            .jwt_refresh_secret,
    )?;

    let refresh_out_token = refresh_token
        .token
        .clone();
    jwt_services::insert_refresh(state, refresh_token).await?;

    Ok((access_token, refresh_out_token))
}

pub async fn refresh(state: &Arc<AppState>, refresh_token: &str) -> Result<String, AppError> {
    if jwt_services::find_refresh_by_token(state, refresh_token)
        .await?
        .is_none()
    {
        return Err(AuthError::TokenInvalid.into());
    }

    let claims = jwt_services::decode_token(
        refresh_token,
        &state
            .env
            .jwt_refresh_secret,
    )?;

    let access_token = jwt_services::generate_token(
        claims.sub,
        chrono::Utc::now()
            .checked_add_signed(jwt_services::ACCESS_TOKEN_EXPIRATION)
            .ok_or(anyhow::Error::msg("Failed to create access token"))?
            .timestamp(),
        &state
            .env
            .jwt_access_secret,
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
    account: Account,
    password: &str,
) -> Result<DeleteResult, AppError> {
    if !verify_password(&account.hashed_password, password) {
        return Err(AuthError::InvalidCredentials.into());
    }

    let deleted_result = jwt_services::delete_many_refresh_by_account_id(state, account.id).await?;
    Ok(deleted_result)
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
            .jwt_access_secret,
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
