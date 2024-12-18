use axum::{
    routing::{delete, get, patch, post},
    Router,
};
use create::create_post;

pub mod create;

pub fn create_posts_router() -> Router {
    Router::new().route("/", post(create_post))
}
