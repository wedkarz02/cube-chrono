use std::sync::Arc;

use axum::Router;
use mongodb::bson::Uuid;
use serde::Deserialize;
use tower_http::trace::TraceLayer;
use validator::Validate;

use crate::AppState;

mod accounts;
pub mod auth;
mod events;
mod hello;
pub mod scrambles;
mod sessions;

#[derive(Deserialize, Validate)]
pub struct PathId {
    id: Uuid,
}

pub fn create_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .nest("/api/v1/hello", hello::create_routes(Arc::clone(&state)))
        .nest(
            "/api/v1/profiles",
            accounts::create_routes(Arc::clone(&state)),
        )
        .nest("/api/v1/auth", auth::create_routes(Arc::clone(&state)))
        .nest("/api/v1/events", events::create_routes(Arc::clone(&state)))
        .nest(
            "/api/v1/scrambles",
            scrambles::create_routes(Arc::clone(&state)),
        )
        .nest(
            "/api/v1/sessions",
            sessions::create_routes(Arc::clone(&state)),
        )
        .layer(TraceLayer::new_for_http())
}
