use axum::{body::Body, extract::Request, http::StatusCode};
use serde_json::json;

use tower::ServiceExt;
use web_api_rust_learn::router::create_main_router;

#[tokio::test]
async fn test_from_request_valid_body() {
    let route = create_main_router();

    let body = Body::from(json!({"text": "Hello, World!"}).to_string());
    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/posts")
        .header("content-type", "application/json")
        .body(body)
        .unwrap();

    let r = route.clone().oneshot(request).await.unwrap();
    assert_eq!(r.status(), StatusCode::OK);
}
