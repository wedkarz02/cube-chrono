use mongodb::bson::Uuid;
use serde::{Deserialize, Serialize};

// NOTE: models should be refactored into domain models, DB entities and endpoint DTOs 
// (or at least just add the separate DTOs for now)

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub struct Event {
    pub id: Uuid,
    pub is_private: bool,
    pub creator_id: Uuid,
    pub title: String,
    pub description: String,
    pub date_timestamp: i64,
    pub moderators: Vec<Uuid>,
    pub participants: Vec<Uuid>,
}

impl Event {
    pub fn new(
        title: &str,
        description: &str,
        date_timestamp: i64,
        creator_id: Uuid,
        is_private: bool,
    ) -> Event {
        Event {
            id: Uuid::new(),
            is_private,
            creator_id,
            title: title.to_owned(),
            description: description.to_owned(),
            date_timestamp,
            moderators: vec![creator_id],
            participants: vec![],
        }
    }

    pub fn add_moderator(&mut self, user_id: Uuid) {
        self.moderators
            .push(user_id);
    }

    pub fn add_participant(&mut self, user_id: Uuid) {
        self.participants
            .push(user_id);
    }
}
