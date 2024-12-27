use std::{error::Error, net::SocketAddr, sync::Arc};

use mongodb::{bson::doc, Client};
use routes::create_routes;
use tokio::signal;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod error;
mod models;
mod routes;
mod services;

#[derive(Debug, Clone)]
pub struct Config {
    pub mongo_uri: String,
    pub mongo_database: String,
    pub backend_port: u16,
    pub jwt_secret: String,
}

impl Config {
    pub fn init() -> Self {
        let mongo_uri = std::env::var("MONGO_URI").expect("MONGO_URI variable should be set");
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET variable should be set");
        let mongo_database = std::env::var("MONGO_INITDB_DATABASE")
            .expect("MONGO_INITDB_DATABASE variable should be set");
        let backend_port = std::env::var("BACKEND_PORT")
            .unwrap_or("8080".into())
            .parse()
            .expect("BACKEND_PORT variable should be a viable port number");

        Config {
            mongo_uri,
            mongo_database,
            backend_port,
            jwt_secret,
        }
    }
}

pub struct AppState {
    client: Client,
    env: Config,
}

pub async fn run(config: Config) -> Result<(), Box<dyn Error>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let client = Client::with_uri_str(&config.mongo_uri).await?;
    client
        .database(&config.mongo_database)
        .run_command(doc! { "ping": 1 })
        .await?;

    tracing::debug!("Connected to MongoDB: {}", config.mongo_database);

    let state = AppState {
        client,
        env: config.clone(),
    };

    let addr: SocketAddr = format!("127.0.0.1:{}", config.backend_port).parse()?;
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::debug!("Listening on: {}", listener.local_addr()?);

    axum::serve(listener, create_routes(Arc::new(state)))
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
