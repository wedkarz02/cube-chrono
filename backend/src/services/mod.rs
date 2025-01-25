use std::sync::Arc;

use mongodb::Collection;

use crate::AppState;

pub mod account_services;
pub mod auth_services;
pub mod jwt_services;
pub mod session_services;
pub mod utils;
pub mod validation_services;

pub struct Collections;

impl Collections {
    pub const ACCOUNTS: &'static str = "accounts";
    pub const REFRESH_TOKENS: &'static str = "refresh_tokens";
    pub const SESSIONS: &'static str = "sessions";
}

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
