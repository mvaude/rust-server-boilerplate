mod router_test;

use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
};
use std::env;
use tower::ServiceExt;

use crate::router::app;
use crate::get_port;

#[tokio::test]
async fn test_health_check() {
    let app = app();

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
    
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    assert_eq!(&body[..], b"OK");
}

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
