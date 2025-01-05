use super::health_check;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;
use crate::router::app;

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
