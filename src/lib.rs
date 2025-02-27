//! A boilerplate Rust server with basic health check functionality
//!
//! This crate provides a simple HTTP server implementation using Axum
//! with a health check endpoint at `/health`.

use axum::Router;
use dotenvy::dotenv;
use std::env;
use std::net::SocketAddr;

pub mod handlers;
pub mod router;

// Re-export app function for convenience
pub use router::app;

/// Get the server port from environment variable or use default
fn get_port() -> u16 {
    env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000)
}

pub fn run(router: Router) {
    dotenv().ok(); // Load .env file if it exists

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let port = get_port();
        let addr = SocketAddr::from(([127, 0, 0, 1], port));
        println!("Server running on {}", addr);

        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, router).await.unwrap();
    });
}

#[cfg(test)]
mod lib_test;
