/// Returns a simple "OK" response to indicate the service is running
/// 
/// # Examples
/// 
/// ```
/// use crate::handlers;
/// 
/// # tokio_test::block_on(async {
/// let response = handlers::health_check().await;
/// assert_eq!(response, "OK");
/// # });
/// ```
pub async fn health_check() -> &'static str {
    "OK"
}
