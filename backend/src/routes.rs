use axum::{http::Method, Router};
use tower_http::cors::{Any, CorsLayer};

mod hello;

pub fn create_routes() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    Router::new()
        .nest("/api/v1/hello", hello::router())
        .layer(cors)
}
