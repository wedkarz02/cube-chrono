use std::sync::Arc;

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};
use axum_extra::json;
use mongodb::bson::doc;
use serde::Deserialize;

use crate::{
    error::{AppError, AuthError},
    models::user::{Role, User},
    services::auth,
    AppState,
};

#[derive(Deserialize)]
pub struct JsonRequest {
    message: String,
}

pub async fn hello_world(Json(body): Json<JsonRequest>) -> impl IntoResponse {
    (
        StatusCode::OK,
        json!({ "message": body.message, "message_from_server": "Hello there traveller"}),
    )
}

pub async fn secret_route(Extension(user): Extension<User>) -> impl IntoResponse {
    (
        StatusCode::IM_A_TEAPOT,
        json!({ "message": format!("Hello {}, I'm a teapot!", user.username) }),
    )
}

pub async fn throw_internal(
    Extension(state): Extension<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> Result<impl IntoResponse, AppError> {
    if !matches!(user.role, Role::Admin) {
        return Err(AuthError::Unauthorized.into());
    }

    if let Err(err) = state
        .client
        .database("invalid/db")
        .collection::<mongodb::bson::Document>("test_collection")
        .insert_one(doc! { "field": "value" })
        .await
    {
        return Err(err.into());
    }

    Ok((
        StatusCode::OK,
        json!({ "message": "This message is impossible to get" }),
    ))
}

pub fn router(state: Arc<AppState>) -> Router {
    let public_routes = Router::new().route("/", post(hello_world));

    let protected_routes = Router::new()
        .route("/secret", get(secret_route))
        .route("/ise", get(throw_internal))
        .layer(axum::middleware::from_fn(auth::auth_guard));

    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .layer(Extension(state))
}
