#![allow(unused)]

use std::sync::Arc;

use axum::{extract::Path, response::IntoResponse, Extension, Json};
use mongodb::{
    bson::{doc, Uuid},
    Collection,
};

use crate::{error::AppError, models::account::Account, AppState};

use super::{get_collection, Collections};

// TODO (wedkarz): change the return type to whatever it should be when used
pub async fn create_user(
    state: &Arc<AppState>,
    account: Account,
) -> Result<impl IntoResponse, AppError> {
    let accounts: Collection<Account> = get_collection(state, Collections::ACCOUNTS);
    let result = accounts
        .insert_one(account)
        .await?;

    Ok(Json(result))
}

pub async fn get_by_id(state: &Arc<AppState>, id: Uuid) -> Result<Account, AppError> {
    let accounts: Collection<Account> = get_collection(state, Collections::ACCOUNTS);
    let result = accounts
        .find_one(doc! { "_id": id })
        .await?;

    let user_body = match result {
        None => return Err(AppError::NotFound),
        Some(u) => u,
    };

    Ok(user_body)
}

// TODO (wedkarz): change the return type to whatever it should be when used
pub async fn update_user(
    state: &Arc<AppState>,
    body: Account,
) -> Result<impl IntoResponse, AppError> {
    let accounts: Collection<Account> = get_collection(state, Collections::ACCOUNTS);
    let result = accounts
        .replace_one(doc! { "_id": body.id }, body)
        .await?;

    Ok(Json(result))
}

// TODO (wedkarz): change the return type to whatever it should be when used
pub async fn delete_user(state: &Arc<AppState>, id: Uuid) -> Result<impl IntoResponse, AppError> {
    let accounts: Collection<Account> = get_collection(state, Collections::ACCOUNTS);
    let result = accounts
        .delete_one(doc! { "_id": id })
        .await?;

    Ok(Json(result))
}
