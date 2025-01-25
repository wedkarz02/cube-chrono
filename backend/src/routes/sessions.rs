use std::sync::Arc;

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Extension, Router,
};
use axum_extra::json;
use mongodb::bson::Uuid;
use serde::Deserialize;
use validator::Validate;

use crate::{
    error::AppError,
    models::{
        account::Account,
        session::{Session, Time},
    },
    services::{
        self, session_services,
        validation_services::{ValidatedJson, ValidatedPath},
    },
    AppState,
};

use super::PathId;

async fn get_all_sessions(
    Extension(state): Extension<Arc<AppState>>,
    Extension(account): Extension<Account>,
) -> Result<impl IntoResponse, AppError> {
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

#[derive(Deserialize, Validate)]
struct EmptySessionPayload {
    #[validate(length(min = 1, max = 32, message = "length must be in range (1..=32)"))]
    name: String,
}

async fn create_empty(
    Extension(state): Extension<Arc<AppState>>,
    Extension(account): Extension<Account>,
    ValidatedJson(payload): ValidatedJson<EmptySessionPayload>,
) -> Result<impl IntoResponse, AppError> {
    let empty_session = Session::new(account.id, &payload.name, &[]);
    let session_id = empty_session.id;
    session_services::create(&state, empty_session).await?;

    Ok((
        StatusCode::CREATED,
        json!({
            "message": "Empty session created",
            "payload": {
                "session_id": session_id,
            }
        }),
    ))
}

#[derive(Deserialize, Validate)]
struct AddTimePayload {
    session_id: Uuid,
    time: Time,
}

async fn insert_time(
    Extension(state): Extension<Arc<AppState>>,
    Extension(account): Extension<Account>,
    ValidatedJson(payload): ValidatedJson<AddTimePayload>,
) -> Result<impl IntoResponse, AppError> {
    let result =
        session_services::insert_time(&state, account.id, payload.session_id, payload.time).await?;
    Ok((
        StatusCode::CREATED,
        json!({
            "message": "New time inserted",
            "payload": {
                "matched_count": result.matched_count,
                "modified_count": result.modified_count,
            }
        }),
    ))
}

pub fn create_routes(state: Arc<AppState>) -> Router {
    let protected_routes = Router::new()
        .route("/", get(get_all_sessions))
        .route("/{id}", get(get_by_id))
        .route("/empty", post(create_empty))
        .route("/add-time", post(insert_time))
        .layer(axum::middleware::from_fn(
            services::auth_services::auth_guard,
        ));

    Router::new()
        .merge(protected_routes)
        .layer(Extension(state))
}
