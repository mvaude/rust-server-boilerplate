use axum::{Router, routing::get};
use std::net::SocketAddr;

pub mod handlers;

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
