//use crate::{database::queries::insert_post, state::AppState};
use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, Request, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};

use crate::{database::queries::insert_post, state::AppState};

#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct InsertPost {
    pub id: i32,
}

pub async fn create_post(
    state: State<AppState>,
    post: CreatePost,
) -> Result<(StatusCode, Json<InsertPost>), (StatusCode, &'static str)> {
    tracing::debug!("Post : {:?}", post);
    let post_id = insert_post(state.db.clone(), &post.text, post.parent_id)
        .await
        .map_err(|error| {
            tracing::error!("Error inserting post into database: {:?}", error);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to insert post into database",
            )
        })?;
    Ok((StatusCode::CREATED, Json(InsertPost { id: post_id })))
}

#[derive(Debug)]
pub struct CreatePost {
    pub text: String,
    pub parent_id: Option<i32>,
}

#[async_trait]
impl<S> FromRequest<S> for CreatePost
where
    Json<CreatePostPartial>: FromRequest<S, Rejection = JsonRejection>,
    S: Send + Sync,
{
    type Rejection = Response;
    async fn from_request(request: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(post) = Json::<CreatePostPartial>::from_request(request, state)
            .await
            .map_err(|error| {
                tracing::error!(
                    "Error extracting json body when creating post {}",
                    error.body_text()
                );
                error.status().into_response()
            })?;

        tracing::debug!("Post : {:?}", post);
        tracing::info!("Post : {:?}", post);
        let Some(text) = post.text else {
            return Err((StatusCode::BAD_REQUEST, "Missing required field 'text'").into_response());
        };

        if text.is_empty() {
            return Err((StatusCode::BAD_REQUEST, "Text cannot be empty").into_response());
        }

        if text.len() > 255 {
            return Err((
                StatusCode::BAD_REQUEST,
                "Text cannot be longer than 1000 characters",
            )
                .into_response());
        }

        if let Some(parent_id) = post.parent_id {
            if parent_id <= 0 {
                return Err((
                    StatusCode::BAD_REQUEST,
                    "Parent ID must be a positive integer",
                )
                    .into_response());
            }
        }
        Ok(Self {
            text,
            parent_id: post.parent_id,
        })
    }
}

#[derive(Deserialize, Debug)]
pub struct CreatePostPartial {
    pub text: Option<String>,
    pub parent_id: Option<i32>,
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        extract::FromRequest,
        http::{Request, StatusCode},
    };
    use serde_json::json;

    use super::*;

    #[tokio::test]
    async fn test_from_request_valid_body() {
        let body = Body::from(json!({"text": "Hello, World!"}).to_string());
        let request = Request::builder()
            .header("content-type", "application/json")
            .body(body)
            .unwrap();

        let result = CreatePost::from_request(request, &()).await;

        assert!(result.is_ok());

        let post = result.unwrap();
        assert_eq!(post.text, "Hello, World!");
    }

    #[tokio::test]
    async fn test_from_request_invalid_body() {
        let body = Body::from(json!({"message": "Invalid"}).to_string());
        let request = Request::builder()
            .header("content-type", "application/json")
            .body(body)
            .unwrap();

        let result = CreatePost::from_request(request, &()).await;

        assert!(result.is_err());

        let response = result.unwrap_err();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_from_request_empty_body() {
        // Create an empty body
        let body = Body::empty();
        let request = Request::builder()
            .header("content-type", "application/json")
            .body(body)
            .unwrap();

        let result = CreatePost::from_request(request, &()).await;

        assert!(result.is_err());

        let response = result.unwrap_err();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_from_request_invalid_field_length() {
        let long_text = "a".repeat(256);
        let body = Body::from(json!({"text": long_text}).to_string());
        let request = Request::builder()
            .header("content-type", "application/json")
            .body(body)
            .unwrap();

        let result = CreatePost::from_request(request, &()).await;

        assert!(result.is_err());

        let response = result.unwrap_err();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_from_request_invalid_parent_id() {
        let body = Body::from(json!({"message": "Hello post ", "parent_id": -1}).to_string());
        let request = Request::builder()
            .header("content-type", "application/json")
            .body(body)
            .unwrap();

        let result = CreatePost::from_request(request, &()).await;

        assert!(result.is_err());

        let response = result.unwrap_err();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
