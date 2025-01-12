use std::process;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use backend::{run, Config};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::debug!("Tracing initialized");

    let config = Config::init();
    tracing::debug!("Environment configuration loaded");

    if let Err(e) = run(config).await {
        tracing::error!("{}", e);
        process::exit(1);
    }
}
