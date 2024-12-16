use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use mongodb::{
    bson::doc,
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Collection,
};

use crate::models::user::User;

pub async fn create_user(
    State(db): State<Collection<User>>,
    Json(body): Json<User>,
) -> Result<Json<InsertOneResult>, (StatusCode, String)> {
    let result = db
        .insert_one(body)
        .await
        .map_err(internal_error)?;

    Ok(Json(result))
}

pub async fn read_user(
    State(db): State<Collection<User>>,
    Path(id): Path<u32>,
) -> Result<Json<Option<User>>, (StatusCode, String)> {
    let result = db
        .find_one(doc! { "_id": id })
        .await
        .map_err(internal_error)?;

    Ok(Json(result))
}

pub async fn update_user(
    State(db): State<Collection<User>>,
    Json(body): Json<User>,
) -> Result<Json<UpdateResult>, (StatusCode, String)> {
    let result = db
        .replace_one(doc! { "_id": body.id }, body)
        .await
        .map_err(internal_error)?;

    Ok(Json(result))
}

pub async fn delete_user(
    State(db): State<Collection<User>>,
    Path(id): Path<u32>,
) -> Result<Json<DeleteResult>, (StatusCode, String)> {
    let result = db
        .delete_one(doc! { "_id": id })
        .await
        .map_err(internal_error)?;

    Ok(Json(result))
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
