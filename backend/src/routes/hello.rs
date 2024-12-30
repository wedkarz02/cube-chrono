use std::sync::Arc;

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};
use axum_extra::json;
use serde::Deserialize;

use crate::{models::user::User, services::auth, AppState};

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

pub fn router(state: Arc<AppState>) -> Router {
    let public_routes = Router::new().route("/", post(hello_world));

    let protected_routes = Router::new()
        .route("/secret", get(secret_route))
        .layer(axum::middleware::from_fn(auth::auth_guard));

    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .layer(Extension(state))
}
