use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, routing::get, Extension, Router};
use axum_extra::json;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    error::AppError,
    services::{scramble_services, validation_services::ValidatedQuery},
    AppState,
};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum ScrambleKind {
    Three,
    // TODO (wedkarz): Add more puzzles later
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Scramble {
    pub kind: ScrambleKind,
    pub sequence: String,
}

#[derive(Clone, Deserialize, Validate)]
struct ScrambleQuery {
    kind: ScrambleKind,
    #[validate(range(min = 1, max = 64, message = "must be in range (1..=64)"))]
    count: usize,
}

async fn generate(
    ValidatedQuery(query): ValidatedQuery<ScrambleQuery>,
) -> Result<impl IntoResponse, AppError> {
    let scrambles: Vec<Scramble> = std::iter::repeat_with(|| {
        scramble_services::generate(
            query
                .kind
                .clone(),
        )
    })
    .take(query.count)
    .collect();

    Ok((
        StatusCode::OK,
        json!({
            "message": &format!("Successfully generated {} new scrambles", scrambles.len()),
            "payload": {
                "scrambles": scrambles,
            }
        }),
    ))
}

pub fn create_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(generate))
        .layer(Extension(state))
}
