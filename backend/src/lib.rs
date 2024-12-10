use std::error::Error;

use mongodb::{bson::doc, Client};
use routes::create_routes;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod routes;

pub async fn run() -> Result<(), Box<dyn Error>> {
    let database_uri =
        std::env::var("DB_URI").unwrap_or_else(|_| "mongodb://127.0.0.1:27017".to_string());
    let client = Client::with_uri_str(database_uri).await?;

    client
        .database("cube-chrono")
        .run_command(doc! { "ping": 1 })
        .await?;

    println!("Connected to mongodb");

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    tracing::debug!("Listening on: {}", listener.local_addr()?);

    axum::serve(listener, create_routes(client)).await?;

    Ok(())
}
