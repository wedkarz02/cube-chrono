use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, routing::get, Extension, Router};
use axum_extra::json;

use crate::{
    error::AppError,
    models::account::Account,
    services::{self, session_services, validation_services::ValidatedPath},
    AppState,
};

use super::PathId;

async fn get_all_sessions(
    Extension(state): Extension<Arc<AppState>>,
    Extension(account): Extension<Account>,
) -> Result<impl IntoResponse, AppError> {
    tracing::debug!("asdfasdf");
    let sessions = session_services::find_all_by_account_id(&state, account.id).await?;
    Ok((
        StatusCode::OK,
        json!({
            "message": &format!("Found {} sessions", sessions.len()),
            "payload": {
                "sessions": sessions,
            }
        }),
    ))
}

async fn get_by_id(
    Extension(state): Extension<Arc<AppState>>,
    Extension(account): Extension<Account>,
    ValidatedPath(path): ValidatedPath<PathId>,
) -> Result<impl IntoResponse, AppError> {
    let session = session_services::find_by_id_and_account_id(&state, account.id, path.id)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok((
        StatusCode::OK,
        json!({
            "message": "Session found",
            "payload": {
                "session": session,
            }
        }),
    ))
}

pub fn create_routes(state: Arc<AppState>) -> Router {
    let protected_routes = Router::new()
        .route("/", get(get_all_sessions))
        .route("/{id}", get(get_by_id))
        .layer(axum::middleware::from_fn(
            services::auth_services::auth_guard,
        ));

    Router::new()
        .merge(protected_routes)
        .layer(Extension(state))
}
