use std::sync::Arc;

use futures::TryStreamExt;
use mongodb::{
    bson::{doc, Uuid},
    Collection,
};

use crate::{error::AppError, models::session::Session, AppState};

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
