use std::error::Error;

use routes::create_routes;

mod routes;

pub async fn run() -> Result<(), Box<dyn Error>> {
    let app = create_routes();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
