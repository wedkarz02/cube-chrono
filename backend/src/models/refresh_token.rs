use mongodb::bson::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct RefreshToken {
    #[serde(rename = "_id")]
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
}

impl RefreshToken {
    pub fn from(user_id: Uuid, token: &str) -> RefreshToken {
        RefreshToken {
            id: Uuid::new(),
            user_id,
            token: token.to_owned(),
        }
    }
}
