use std::process;

use backend::{run, Config};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let config = Config::init();

    if let Err(e) = run(config).await {
        eprintln!("{}", e);
        process::exit(1);
    }
}
