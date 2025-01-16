use mongodb::bson::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
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
    pub roles: Vec<Role>,
}

impl User {
    pub fn from(username: &str, hashed_password: &str, roles: &[Role]) -> User {
        User {
            id: Uuid::new(),
            username: username.to_owned(),
            hashed_password: hashed_password.to_owned(),
            roles: roles.to_owned(),
        }
    }

    pub fn privileged(&self, perm: Role) -> bool {
        self.roles
            .contains(&perm)
    }
}
