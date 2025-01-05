use super::health_check;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;
use crate::router::app;

#[tokio::test]
async fn test_health_check_function() {
    let response = health_check().await;
    assert_eq!(response, "OK");
}

#[tokio::test]
async fn test_health_check_endpoint() {
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
    
    let body = axum::body::to_bytes(response.into_body()).await.unwrap();
    assert_eq!(&body[..], b"OK");
}
