use std::sync::Arc;

use axum::Extension;
use axum::{routing::post, Router};

use crate::services;
use crate::AppState;

pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/register", post(services::auth::register))
        .route("/login", post(services::auth::login))
        .route("/logout", post(services::auth::logout))
        .route("/refresh", post(services::auth::refresh))
        .layer(Extension(state))
}
