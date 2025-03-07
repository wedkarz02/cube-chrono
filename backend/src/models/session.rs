use mongodb::bson::Uuid;
use serde::{Deserialize, Serialize};

use crate::routes::scrambles::Scramble;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Time {
    pub millis: u64,
    pub recorded_at: u64,
    pub scramble: Scramble,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Session {
    #[serde(rename = "_id")]
    pub id: Uuid,
    pub account_id: Uuid,
    pub name: String,
    pub times: Vec<Time>,
}

impl Session {
    pub fn new(account_id: Uuid, name: &str, times: &[Time]) -> Session {
        Session {
            id: Uuid::new(),
            account_id,
            name: name.to_owned(),
            times: times.to_owned(),
        }
    }
}
