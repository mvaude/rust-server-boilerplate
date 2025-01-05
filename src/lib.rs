//! A boilerplate Rust server with basic health check functionality
//! 
//! This crate provides a simple HTTP server implementation using Axum
//! with a health check endpoint at `/health`.

use axum::Router;
use std::net::SocketAddr;
use std::env;
use dotenv::dotenv;

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
        
        axum::Server::bind(&addr)
            .serve(router.into_make_service())
            .await
            .unwrap();
    });
}

#[cfg(test)]
mod tests;

#[cfg(test)]
mod test {
    use super::*;
    use std::env;

    #[test]
    fn test_port_configuration() {
        // Test default port
        assert_eq!(get_port(), 3000);

        // Test custom port
        env::set_var("PORT", "8080");
        assert_eq!(get_port(), 8080);

        // Test invalid port falls back to default
        env::set_var("PORT", "invalid");
        assert_eq!(get_port(), 3000);
    }
}
