use mongodb::bson::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Role {
    User,
    Admin,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: Uuid,
    pub username: String,
    pub hashed_password: String,
    pub role: Role,
}
