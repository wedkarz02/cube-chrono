use crate::error::AppError;
use crate::models::event::Event;
use crate::AppState;
use axum::response::IntoResponse;
use axum::routing::{delete, get, post, put};
use axum::{Extension, Json, Router};
use std::sync::Arc;
use axum::extract::Path;
use mongodb::bson::Uuid;

pub fn create_routes(state: Arc<AppState>) -> Router {
    Router::new()
        // .route("/", get(get_all))
        // .route("/", post(create))
        // .route("/{id}", get(get_one))
        // .route("/{id}", put(update))
        // .route("/{id}", delete(delete_one))
        .layer(Extension(state))
}

// async fn get_all(
//     Extension(_state): Extension<Arc<AppState>>,
// ) -> Result<impl IntoResponse, AppError> {
//     Err(AppError::NotImplemented) // FIXME: "type annotations needed" - not working as expected
//     // TODO: get all non-private events
// }
// 
// async fn create(
//     Extension(_state): Extension<Arc<AppState>>,
//     Json(_payload): Json<Event>,
// ) -> Result<impl IntoResponse, AppError> {
//     Err(AppError::NotImplemented)
//     // TODO: create a new event (payload shouldn't be of type Event but some DTO instead)
// }
// 
// async fn get_one(
//     Extension(_state): Extension<Arc<AppState>>,
//     Path(_id): Path<Uuid>,
// ) -> Result<impl IntoResponse, AppError> {
//     Err(AppError::NotImplemented)
//     // TODO: get a single event by id (if it's private, should be authorized to see it)
// }
// 
// async fn update(
//     Extension(_state): Extension<Arc<AppState>>,
//     Path(_id): Path<Uuid>,
//     Json(_payload): Json<Event>,
// ) -> Result<impl IntoResponse, AppError> {
//     Err(AppError::NotImplemented)
//     // TODO: update a single event by id (should be authorized)
// }
// 
// async fn delete_one(
//     Extension(_state): Extension<Arc<AppState>>,
//     Path(_id): Path<Uuid>,
// ) -> Result<impl IntoResponse, AppError> {
//     Err(AppError::NotImplemented)
//     // TODO: delete a single event by id (should be authorized)
// }
