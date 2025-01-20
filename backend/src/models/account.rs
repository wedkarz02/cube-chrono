use mongodb::bson::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum Role {
    User,
    Admin,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Account {
    #[serde(rename = "_id")]
    pub id: Uuid,
    pub username: String,
    pub hashed_password: String,
    pub roles: Vec<Role>,
}

impl Account {
    pub fn new(username: &str, hashed_password: &str, roles: &[Role]) -> Account {
        Account {
            id: Uuid::new(),
            username: username.to_owned(),
            hashed_password: hashed_password.to_owned(),
            roles: roles.to_owned(),
        }
    }

    pub fn has_role(&self, role: Role) -> bool {
        self.roles
            .contains(&role)
    }
}
