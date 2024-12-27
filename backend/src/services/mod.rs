use std::sync::Arc;

use mongodb::Collection;

use crate::AppState;

pub mod auth;
pub mod jwt;
pub mod user;

pub fn get_collection<T: Send + Sync>(state: &Arc<AppState>, name: &str) -> Collection<T> {
    state
        .client
        .database(
            &state
                .env
                .mongo_database,
        )
        .collection(name)
}
