use std::sync::Arc;

use axum::Router;
use tower_http::trace::TraceLayer;

use crate::AppState;

mod accounts;
pub mod auth;
mod hello;

pub fn create_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .nest("/api/v1/hello", hello::create_routes(Arc::clone(&state)))
        .nest(
            "/api/v1/profiles",
            accounts::create_routes(Arc::clone(&state)),
        )
        .nest("/api/v1/auth", auth::create_routes(Arc::clone(&state)))
        .layer(TraceLayer::new_for_http())
}
