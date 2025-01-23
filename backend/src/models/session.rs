#![allow(unused)]

use mongodb::bson::Uuid;
use serde::{Deserialize, Serialize};

use crate::routes::scrambles::Scramble;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Time {
    millis: u64,
    recorded_at: u64,
    scramble: Scramble,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Session {
    #[serde(rename = "_id")]
    pub id: Uuid,
    pub account_id: Uuid,
    pub times: Vec<Time>,
}

impl Session {
    pub fn new(account_id: Uuid, times: &[Time]) -> Session {
        Session {
            id: Uuid::new(),
            account_id,
            times: times.to_owned(),
        }
    }
}
