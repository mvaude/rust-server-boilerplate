//! A boilerplate Rust server with basic health check functionality
//! 
//! This crate provides a simple HTTP server implementation using Axum
//! with a health check endpoint at `/health`.
//! 
//! # Examples
//! 
//! ```no_run
//! use rust_server_boilerplate::{app, run};
//! 
//! fn main() {
//!     let app = app();
//!     run(app);
//! }
//! ```

use axum::{Router, routing::get};
use std::net::SocketAddr;

pub mod handlers;

/// Creates and returns the application router with configured routes
/// 
/// # Examples
/// 
/// ```
/// use rust_server_boilerplate::app;
/// 
/// let router = app();
/// ```
pub fn app() -> Router {
    Router::new()
        .route("/health", get(handlers::health_check))
}

pub fn run(router: Router) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
        println!("Server running on {}", addr);
        
        axum::Server::bind(&addr)
            .serve(router.into_make_service())
            .await
            .unwrap();
    });
}

#[cfg(test)]
mod tests;
