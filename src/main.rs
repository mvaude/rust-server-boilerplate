use axum::{Router, routing::get};
use std::net::SocketAddr;

mod handlers;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(handlers::health_check));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on {}", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
