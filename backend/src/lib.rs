use std::{error::Error, net::SocketAddr};

use mongodb::{bson::doc, Client};
use routes::create_routes;
use tokio::signal;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod models;
mod routes;
mod services;

pub async fn run() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database_uri = std::env::var("MONGO_URI")?;
    let client = Client::with_uri_str(database_uri).await?;

    client
        .database("cube-chrono")
        .run_command(doc! { "ping": 1 })
        .await?;

    tracing::debug!("Connected to MongoDB");

    let port: u16 = std::env::var("BACKEND_PORT")
        .unwrap_or("8080".into())
        .parse()?;

    let addr: SocketAddr = format!("127.0.0.1:{}", port).parse()?;
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::debug!("Listening on: {}", listener.local_addr()?);

    axum::serve(listener, create_routes(client))
        .with_graceful_shutdown(async {
            signal::ctrl_c()
                .await
                .expect("Failed to install the Ctrl-C handler");
            tracing::debug!("Shutdown signal received");
        })
        .await?;

    tracing::debug!("Graceful shutdown.");
    Ok(())
}
