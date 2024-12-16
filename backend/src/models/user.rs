use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: u32,
    pub username: String,
    pub password: String,
}
