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
    models::account::{Account, Role},
    services::auth_services,
    AppState,
};

#[derive(Deserialize)]
pub struct JsonRequest {
    message: String,
}

async fn hello_world(Json(body): Json<JsonRequest>) -> impl IntoResponse {
    (
        StatusCode::OK,
        json!({
            "message": body.message,
            "payload": {
                "message_from_server": "Hello there traveller"
            }
        }),
    )
}

async fn secret_route(Extension(account): Extension<Account>) -> impl IntoResponse {
    (
        StatusCode::IM_A_TEAPOT,
        json!({ "message": format!("Hello {}, I'm a teapot!", account.username) }),
    )
}

async fn throw_internal(
    Extension(state): Extension<Arc<AppState>>,
    Extension(account): Extension<Account>,
) -> Result<impl IntoResponse, AppError> {
    if !account.has_role(Role::Admin) {
        return Err(AuthError::Forbidden.into());
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

pub fn create_routes(state: Arc<AppState>) -> Router {
    let public_routes = Router::new().route("/", post(hello_world));

    let protected_routes = Router::new()
        .route("/secret", get(secret_route))
        .route("/ise", get(throw_internal))
        .layer(axum::middleware::from_fn(auth_services::auth_guard));

    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .layer(Extension(state))
}
