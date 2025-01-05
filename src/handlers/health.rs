/// Returns a simple "OK" response to indicate the service is running
/// 
/// # Examples
/// 
/// ```
/// # use rust_server_boilerplate::handlers;
/// # tokio_test::block_on(async {
/// let response = handlers::health_check().await;
/// assert_eq!(response, "OK");
/// # });
/// ```
pub async fn health_check() -> &'static str {
    "OK"
}

#[cfg(test)]
mod tests {
    use super::*;
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
}
