use std::process;

use backend::run;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    if let Err(e) = run().await {
        eprintln!("{}", e);
        process::exit(1);
    }
}
