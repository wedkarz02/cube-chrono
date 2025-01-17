#![allow(unused)]

use std::sync::Arc;

use axum::{extract::Path, response::IntoResponse, Extension, Json};
use mongodb::{
    bson::{doc, Uuid},
    Collection,
};

use crate::{error::AppError, models::user::User, AppState};

use super::get_collection;

// TODO (wedkarz): change the return type to whatever it should be when used
pub async fn create_user(state: &Arc<AppState>, user: User) -> Result<impl IntoResponse, AppError> {
    let users: Collection<User> = get_collection(state, "users");
    let result = users
        .insert_one(user)
        .await?;

    Ok(Json(result))
}

pub async fn get_by_id(state: &Arc<AppState>, id: Uuid) -> Result<User, AppError> {
    let users: Collection<User> = get_collection(state, "users");
    let result = users
        .find_one(doc! { "_id": id })
        .await?;

    let user_body = match result {
        None => return Err(AppError::NotFound),
        Some(u) => u,
    };

    Ok(user_body)
}

// TODO (wedkarz): change the return type to whatever it should be when used
pub async fn update_user(state: &Arc<AppState>, body: User) -> Result<impl IntoResponse, AppError> {
    let users: Collection<User> = get_collection(state, "users");
    let result = users
        .replace_one(doc! { "_id": body.id }, body)
        .await?;

    Ok(Json(result))
}

// TODO (wedkarz): change the return type to whatever it should be when used
pub async fn delete_user(state: &Arc<AppState>, id: Uuid) -> Result<impl IntoResponse, AppError> {
    let users: Collection<User> = get_collection(state, "users");
    let result = users
        .delete_one(doc! { "_id": id })
        .await?;

    Ok(Json(result))
}
