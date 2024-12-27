use std::sync::Arc;

use axum::Router;
use tower_http::trace::TraceLayer;

use crate::AppState;

pub mod auth;
mod hello;
mod user;

pub fn create_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .nest("/api/v1/hello", hello::router(state.clone()))
        .nest("/api/v1/user", user::router(state.clone()))
        .nest("/api/v1/auth", auth::router(state.clone()))
        .layer(TraceLayer::new_for_http())
}
