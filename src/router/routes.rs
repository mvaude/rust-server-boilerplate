use axum::{Router, routing::get};
use crate::handlers;

/// Creates and returns the application router with configured routes
/// 
/// # Examples
/// 
/// ```
/// # use rust_server_boilerplate::router;
/// let app = router::app();
/// ```
pub fn app() -> Router {
    Router::new()
        .route("/health", get(handlers::health_check))
}
