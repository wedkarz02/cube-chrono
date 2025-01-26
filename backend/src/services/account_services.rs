use std::sync::Arc;

use futures::TryStreamExt;
use mongodb::{
    bson::{doc, Uuid},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Collection,
};

use crate::{error::AppError, models::account::Account, AppState};

use super::{get_collection, Collections};

pub async fn insert(state: &Arc<AppState>, account: Account) -> Result<InsertOneResult, AppError> {
    let accounts: Collection<Account> = get_collection(state, Collections::ACCOUNTS);
    let result = accounts
        .insert_one(account)
        .await?;

    Ok(result)
}

pub async fn find_all(state: &Arc<AppState>) -> Result<Vec<Account>, AppError> {
    let accounts: Collection<Account> = get_collection(state, Collections::ACCOUNTS);
    let result: Vec<Account> = accounts
        .find(doc! {})
        .await?
        .try_collect()
        .await?;

    Ok(result)
}

pub async fn find_by_id(state: &Arc<AppState>, id: Uuid) -> Result<Option<Account>, AppError> {
    let accounts: Collection<Account> = get_collection(state, Collections::ACCOUNTS);
    let result = accounts
        .find_one(doc! { "_id": id })
        .await?;

    Ok(result)
}

pub async fn find_by_username(
    state: &Arc<AppState>,
    username: &str,
) -> Result<Option<Account>, AppError> {
    let accounts: Collection<Account> = get_collection(state, Collections::ACCOUNTS);
    let result = accounts
        .find_one(doc! { "username": username })
        .await?;

    Ok(result)
}

pub async fn update(state: &Arc<AppState>, body: Account) -> Result<UpdateResult, AppError> {
    let accounts: Collection<Account> = get_collection(state, Collections::ACCOUNTS);
    let result = accounts
        .replace_one(doc! { "_id": body.id }, body)
        .await?;

    Ok(result)
}

pub async fn delete_by_id(state: &Arc<AppState>, id: Uuid) -> Result<DeleteResult, AppError> {
    let accounts: Collection<Account> = get_collection(state, Collections::ACCOUNTS);
    let result = accounts
        .delete_one(doc! { "_id": id })
        .await?;

    Ok(result)
}
