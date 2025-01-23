use crate::models::event::Event;
use mongodb::bson::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum Role {
    User,
    EventModerator(Event),
    Admin,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
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

    #[allow(unused)]
    pub fn is_event_moderator(&self, event_id: Uuid) -> bool {
        self.roles
            .iter()
            .any(|role| {
                if let Role::EventModerator(event) = role {
                    event.id == event_id
                } else {
                    false
                }
            })
    }
}
