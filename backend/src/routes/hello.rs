use axum::{routing::post, Json, Router};
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
        message_from_server: "Hello from server, lorem ipsum".to_owned(),
    })
}

pub fn router() -> Router {
    Router::new().route("/", post(hello_world))
}
