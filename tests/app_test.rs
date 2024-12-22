use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use serde_json::{json, Value};

use http_body_util::BodyExt; // for `collect`
use tower::ServiceExt;
use web_api_rust_learn::{
    database::connect::connect_to_database,
    router::{create_main_router, posts::create::InsertPost},
    state::AppState,
};

#[tokio::test]
async fn test_from_request_valid_body() {
    let body = Body::from(json!({"text": "Hello, World!"}).to_string());
    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/posts")
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .body(body)
        .unwrap();

    let database_uri = "postgres://postgres:postgres@localhost:5432/postgres";
    let db = connect_to_database(&database_uri)
        .await
        .expect("Failed to connect to database");
    let state = AppState { db };
    let router = create_main_router(state);

    let r = router.clone().oneshot(request).await.unwrap();
    assert_eq!(r.status(), StatusCode::CREATED);
    let body = r.into_body().collect().await.unwrap().to_bytes();
    let body: InsertPost = serde_json::from_slice(&body).unwrap();

    tracing::info!("Body: {:?}", body);
}
