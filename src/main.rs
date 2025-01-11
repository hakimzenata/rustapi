use axum::{Router, Server};
use std::net::SocketAddr;
use my_api::create_router; // Assuming the new file is named my_api.rs

use dotenvy::dotenv;
mod router;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match dotenv() {
        Ok(_) => println!("Loaded .env file"),
        Err(e) => println!("Failed to load .env file: {}", e),
    }
    let server = format!("{}:{}", dotenvy::var("HOST").unwrap(), dotenvy::var("PORT").unwrap());
    println!("Listening on {}", server);
    let app = create_app(server).await;
    match app {
        Ok(_) => println!("Server started successfully"),
        Err(e) => println!("Failed to start server: {}", e),
    }
    Ok(())
}
