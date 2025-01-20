use std::{net::SocketAddr, sync::Arc};

use mongodb::{bson::doc, Client};
use routes::create_routes;
use tokio::signal;

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
    pub superuser_password: String,
}

impl Config {
    pub fn init() -> Self {
        let mongo_uri = std::env::var("MONGO_URI").expect("MONGO_URI variable should be set");
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET variable should be set");
        let superuser_password =
            std::env::var("SUPERUSER_PASSWORD").expect("SUPERUSER_PASSWORD variable should be set");
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
            superuser_password,
        }
    }
}

pub struct AppState {
    client: Client,
    env: Config,
}

pub async fn run(config: Config) -> anyhow::Result<()> {
    let client = Client::with_uri_str(&config.mongo_uri).await?;
    client
        .database(&config.mongo_database)
        .run_command(doc! { "ping": 1 })
        .await?;

    tracing::debug!("Connected to MongoDB: {}", config.mongo_database);

    let state = Arc::new(AppState {
        client,
        env: config,
    });

    let superuser = services::auth_services::create_super_user(&Arc::clone(&state)).await?;
    tracing::info!(
        "SuperUser initialized with: (username: {}, password: {})",
        superuser.username,
        &state
            .env
            .superuser_password
    );

    let addr: SocketAddr = format!(
        "127.0.0.1:{}",
        state
            .env
            .backend_port
    )
    .parse()?;
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::debug!("Listening on: {}", listener.local_addr()?);

    axum::serve(listener, create_routes(Arc::clone(&state)))
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
