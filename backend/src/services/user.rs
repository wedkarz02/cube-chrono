use std::sync::Arc;

use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use mongodb::{
    bson::{doc, Uuid},
    Collection,
};

use crate::{error::internal_error, models::user::User, AppState};

use super::get_collection;

pub async fn create_user(
    Extension(state): Extension<Arc<AppState>>,
    Json(body): Json<User>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let users: Collection<User> = get_collection(&state, "users");
    let result = users
        .insert_one(body)
        .await
        .map_err(internal_error)?;

    Ok(Json(result))
}

pub async fn read_user(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let users: Collection<User> = get_collection(&state, "users");
    let result = users
        .find_one(doc! { "_id": id })
        .await
        .map_err(internal_error)?;

    Ok(Json(result))
}

pub async fn update_user(
    Extension(state): Extension<Arc<AppState>>,
    Json(body): Json<User>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let users: Collection<User> = get_collection(&state, "users");
    let result = users
        .replace_one(doc! { "_id": body.id }, body)
        .await
        .map_err(internal_error)?;

    Ok(Json(result))
}

pub async fn delete_user(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let users: Collection<User> = get_collection(&state, "users");
    let result = users
        .delete_one(doc! { "_id": id })
        .await
        .map_err(internal_error)?;

    Ok(Json(result))
}
