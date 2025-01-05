use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
};
use tower::ServiceExt;

use super::app;

#[tokio::test]
async fn test_router_creation() {
    let app = app();
    // The fact that this compiles proves it's a Router
    let _: Router = app;
}

#[tokio::test]
async fn test_router_health_endpoint() {
    let app = app();

    // Test health endpoint exists and returns correct response
    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_router_unknown_endpoint() {
    let app = app();

    // Test non-existent endpoint returns 404
    let response = app
        .oneshot(
            Request::builder()
                .uri("/nonexistent")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
