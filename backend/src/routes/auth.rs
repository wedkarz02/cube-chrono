use std::sync::Arc;

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{routing::post, Router};
use axum::{Extension, Json};
use axum_extra::json;
use serde::Deserialize;
use validator::Validate;

use crate::error::AppError;
use crate::models::account::Role;
use crate::services::validation_services::ValidatedJson;
use crate::services::{self, validation_services};
use crate::AppState;

#[derive(Deserialize, Validate)]
pub struct AuthPayload {
    #[validate(length(min = 4, max = 32, message = "length must be in range (4..=32)"))]
    #[validate(custom(function = "validation_services::ascii_string"))]
    pub username: String,
    #[validate(custom(function = "validation_services::strong_password"))]
    pub password: String,
}

#[derive(Deserialize)]
pub struct RefreshPayload {
    pub refresh_token: String,
}

async fn register(
    Extension(state): Extension<Arc<AppState>>,
    ValidatedJson(payload): ValidatedJson<AuthPayload>,
) -> Result<impl IntoResponse, AppError> {
    let new_account = services::auth_services::register(&state, payload, &[Role::User]).await?;

    Ok((
        StatusCode::CREATED,
        json!({
            "message": "Account created",
            "paylaod": {
                "created_account": new_account
            }
        }),
    ))
}

async fn login(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<AuthPayload>,
) -> Result<impl IntoResponse, AppError> {
    let (access_token, refresh_token) = services::auth_services::login(&state, payload).await?;

    Ok((
        StatusCode::OK,
        json!({
            "message": "Login successful",
            "payload": {
                "access_token": access_token,
                "refresh_token": refresh_token
            }
        }),
    ))
}

async fn logout(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<RefreshPayload>,
) -> Result<impl IntoResponse, AppError> {
    let logout_message = services::auth_services::logout(&state, payload.refresh_token).await?;
    Ok((StatusCode::OK, json!({ "message": logout_message })))
}

async fn refresh(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<RefreshPayload>,
) -> Result<impl IntoResponse, AppError> {
    let access_token = services::auth_services::refresh(&state, payload.refresh_token).await?;
    Ok((
        StatusCode::OK,
        json!({
            "message": "Token refreshed",
            "payload": {
                "access_token": access_token
            }
        }),
    ))
}

async fn revoke_all_sessions(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<AuthPayload>,
) -> Result<impl IntoResponse, AppError> {
    let revoked_refresh_tokens =
        services::auth_services::revoke_all_refresh_tokens(&state, payload.username).await?;
    let message = if revoked_refresh_tokens > 0 {
        &format!(
            "Successfully revoked all ({}) sessions",
            revoked_refresh_tokens
        )
    } else {
        "No sessions to revoke"
    };
    Ok((
        StatusCode::OK,
        json!({
            "message": message,
            "payload": {
                "revoked_sessions": revoked_refresh_tokens
            }
        }),
    ))
}

pub fn create_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/refresh", post(refresh))
        .route("/revoke-all-sessions", post(revoke_all_sessions))
        .layer(Extension(state))
}
