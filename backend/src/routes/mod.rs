use axum::Router;
use mongodb::Client;
use tower_http::trace::TraceLayer;

mod hello;
mod user;

pub fn create_routes(client: Client) -> Router {
    Router::new()
        .nest("/api/v1/hello", hello::router())
        .nest("/api/v1/user", user::router(&client))
        .layer(TraceLayer::new_for_http())
}
