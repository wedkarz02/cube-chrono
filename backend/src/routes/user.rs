use std::sync::Arc;

use axum::{
    routing::{delete, get, post, put},
    Extension, Router,
};

use crate::{services, AppState};

pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", post(services::user::create_user))
        .route("/:id", get(services::user::read_user))
        .route("/", put(services::user::update_user))
        .route("/:id", delete(services::user::delete_user))
        .layer(Extension(state))
}
