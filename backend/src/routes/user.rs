use axum::{
    routing::{delete, get, post, put},
    Router,
};
use mongodb::{Client, Collection};

use crate::models::user::User;
use crate::services;

pub fn router(client: &Client) -> Router {
    let collection: Collection<User> = client
        .database("cube-chrono")
        .collection("users");

    Router::new()
        .route("/create", post(services::user::create_user))
        .route("/read/:id", get(services::user::read_user))
        .route("/update", put(services::user::update_user))
        .route("/delete/:id", delete(services::user::delete_user))
        .with_state(collection)
}
