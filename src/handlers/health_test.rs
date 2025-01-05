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
    
    let body = response.into_body();
    let bytes = axum::body::to_bytes(body, usize::MAX).await.unwrap();
    assert_eq!(&bytes[..], b"OK");
}
