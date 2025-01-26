use std::sync::Arc;

use futures::TryStreamExt;
use mongodb::{
    bson::{doc, Uuid},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Collection,
};

use crate::{
    error::AppError,
    models::session::{Session, Time},
    AppState,
};

use super::{get_collection, Collections};

pub async fn find_all_by_account_id(
    state: &Arc<AppState>,
    id: Uuid,
) -> Result<Vec<Session>, AppError> {
    let sessions: Collection<Session> = get_collection(state, Collections::SESSIONS);
    let result = sessions
        .find(doc! { "account_id": id })
        .await?
        .try_collect()
        .await?;

    Ok(result)
}

pub async fn find_by_id_and_account_id(
    state: &Arc<AppState>,
    account_id: Uuid,
    id: Uuid,
) -> Result<Option<Session>, AppError> {
    let sessions: Collection<Session> = get_collection(state, Collections::SESSIONS);
    let result = sessions
        .find_one(doc! { "_id": id, "account_id": account_id })
        .await?;

    Ok(result)
}

pub async fn create(state: &Arc<AppState>, session: Session) -> Result<InsertOneResult, AppError> {
    let sessions: Collection<Session> = get_collection(state, Collections::SESSIONS);
    let result = sessions
        .insert_one(session)
        .await?;

    Ok(result)
}

pub async fn update_by_id_and_account_id(
    state: &Arc<AppState>,
    account_id: Uuid,
    id: Uuid,
    body: Session,
) -> Result<UpdateResult, AppError> {
    let sessions: Collection<Session> = get_collection(state, Collections::SESSIONS);
    let result = sessions
        .replace_one(doc! { "_id": id, "account_id": account_id }, body)
        .await?;

    Ok(result)
}

pub async fn insert_time(
    state: &Arc<AppState>,
    account_id: Uuid,
    id: Uuid,
    time: Time,
) -> Result<UpdateResult, AppError> {
    let mut session = match find_by_id_and_account_id(state, account_id, id).await? {
        None => return Err(AppError::NotFound),
        Some(s) => s,
    };

    session
        .times
        .push(time);

    update_by_id_and_account_id(state, account_id, id, session).await
}

pub async fn delete_by_id_and_account_id(
    state: &Arc<AppState>,
    account_id: Uuid,
    id: Uuid,
) -> Result<DeleteResult, AppError> {
    let sessions: Collection<Session> = get_collection(state, Collections::SESSIONS);
    let result = sessions
        .delete_one(doc! { "_id": id, "account_id": account_id })
        .await?;

    Ok(result)
}

pub async fn delete_all_by_account_id(
    state: &Arc<AppState>,
    account_id: Uuid,
) -> Result<DeleteResult, AppError> {
    let sessions: Collection<Session> = get_collection(state, Collections::SESSIONS);
    let result = sessions
        .delete_many(doc! { "account_id": account_id })
        .await?;

    Ok(result)
}
