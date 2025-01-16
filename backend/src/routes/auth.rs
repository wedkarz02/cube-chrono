use std::sync::Arc;

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{routing::post, Router};
use axum::{Extension, Json};
use axum_extra::json;
use serde::Deserialize;

use crate::error::AppError;
use crate::models::user::Role;
use crate::services;
use crate::AppState;

#[derive(Deserialize)]
pub struct AuthPayload {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RefreshPayload {
    pub refresh_token: String,
}

pub async fn register(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<AuthPayload>,
) -> Result<impl IntoResponse, AppError> {
    let new_user = services::auth::register(&state, payload, &[Role::User]).await?;

    Ok((
        StatusCode::CREATED,
        json!({ "message": "user created", "data": { "new_user": new_user } }),
    ))
}

pub async fn login(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<AuthPayload>,
) -> Result<impl IntoResponse, AppError> {
    let (access_token, refresh_token) = services::auth::login(&state, payload).await?;

    Ok((
        StatusCode::OK,
        json!({ "message": "logged in", "data": { "access_token": access_token, "refresh_token": refresh_token }}),
    ))
}

pub async fn logout(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<RefreshPayload>,
) -> Result<impl IntoResponse, AppError> {
    let logout_message = services::auth::logout(&state, payload.refresh_token).await?;
    Ok((StatusCode::OK, json!({ "message": logout_message })))
}

pub async fn refresh(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<RefreshPayload>,
) -> Result<impl IntoResponse, AppError> {
    let access_token = services::auth::refresh(&state, payload.refresh_token).await?;
    Ok((
        StatusCode::OK,
        json!({ "message": "token refreshed", "data": { "access_token": access_token }}),
    ))
}

pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/refresh", post(refresh))
        .layer(Extension(state))
}
