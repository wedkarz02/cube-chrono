use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use mongodb::{
    bson::doc,
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Client, Collection,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct JsonRequest {
    message: String,
}

#[derive(Serialize)]
pub struct JsonResponse {
    message: String,
    message_from_server: String,
}

pub async fn hello_world(Json(body): Json<JsonRequest>) -> Json<JsonResponse> {
    Json(JsonResponse {
        message: body.message,
        message_from_server: "Hello from server".to_owned(),
    })
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    #[serde(rename = "_id")]
    id: u32,
    username: String,
    password: String,
}

async fn create_user(
    State(db): State<Collection<User>>,
    Json(body): Json<User>,
) -> Result<Json<InsertOneResult>, (StatusCode, String)> {
    let result = db
        .insert_one(body)
        .await
        .map_err(internal_error)?;

    Ok(Json(result))
}

async fn read_user(
    State(db): State<Collection<User>>,
    Path(id): Path<u32>,
) -> Result<Json<Option<User>>, (StatusCode, String)> {
    let result = db
        .find_one(doc! { "_id": id })
        .await
        .map_err(internal_error)?;

    Ok(Json(result))
}

async fn update_user(
    State(db): State<Collection<User>>,
    Json(body): Json<User>,
) -> Result<Json<UpdateResult>, (StatusCode, String)> {
    let result = db
        .replace_one(doc! { "_id": body.id }, body)
        .await
        .map_err(internal_error)?;

    Ok(Json(result))
}

async fn delete_user(
    State(db): State<Collection<User>>,
    Path(id): Path<u32>,
) -> Result<Json<DeleteResult>, (StatusCode, String)> {
    let result = db
        .delete_one(doc! { "_id": id })
        .await
        .map_err(internal_error)?;

    Ok(Json(result))
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

pub fn router(client: &Client) -> Router {
    let collection: Collection<User> = client
        .database("cube-chrono")
        .collection("users");

    Router::new()
        .route("/", post(hello_world))
        .route("/user/create", post(create_user))
        .route("/user/read/:id", get(read_user))
        .route("/user/update", put(update_user))
        .route("/user/delete/:id", delete(delete_user))
        .with_state(collection)
}
