use axum::{routing::get, Router};

pub fn create_router(server: String) -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new().route("/api", get(api_handler));
    Ok(())
}

async fn api_handler() -> &'static str {
    "Hello, this is the API response!"
}

struct ApiResponse {
    timestamp: String,
    message: String,
}

pub async fn create_app(server: String) -> Result<(), Box<dyn std::error::Error>> {
    let listener = tokio::net::TcpListener::bind(server.parse()?).await?;
    let app = create_router(server).await.into_make_service();

    // Start the server here using serve
    axum::Server::from_tcp(listener, app).serve().await?;

    Ok(())
}
