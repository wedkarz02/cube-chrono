#![allow(unused)]

use std::sync::Arc;

use axum::{response::IntoResponse, routing::get, Extension, Router};
use serde::Deserialize;
use validator::Validate;

use crate::{error::AppError, services::validation_services::ValidatedQuery, AppState};

#[derive(Deserialize)]
pub enum ScrambleKind {
    Three,
    // TODO (wedkarz): Add more puzzles later
}

#[derive(Deserialize, Validate)]
struct ScrambleQuery {
    kind: ScrambleKind,
    count: u8,
}

async fn generate(
    ValidatedQuery(_query): ValidatedQuery<ScrambleQuery>,
) -> Result<impl IntoResponse, AppError> {
    Err::<AppError, _>(AppError::NotImplemented)
}

pub fn create_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(generate))
        .layer(Extension(state))
}
