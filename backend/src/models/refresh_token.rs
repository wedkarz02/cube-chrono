use mongodb::bson::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct RefreshToken {
    #[serde(rename = "_id")]
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
}
