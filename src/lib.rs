//! A boilerplate Rust server with basic health check functionality
//! 
//! This crate provides a simple HTTP server implementation using Axum
//! with a health check endpoint at `/health`.

use axum::Router;
use std::net::SocketAddr;

pub mod handlers;
pub mod router;

// Re-export app function for convenience
pub use router::app;

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
