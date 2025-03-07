use mongodb::bson::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct RefreshToken {
    #[serde(rename = "_id")]
    pub id: Uuid,
    pub account_id: Uuid,
    pub expiry_timestamp: i64,
    pub token: String,
}

impl RefreshToken {
    pub fn new(account_id: Uuid, expiry_timestamp: i64, token: &str) -> RefreshToken {
        RefreshToken {
            id: Uuid::new(),
            account_id,
            expiry_timestamp,
            token: token.to_owned(),
        }
    }
}
