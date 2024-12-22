use axum::{
    routing::{delete, get, patch, post},
    Router,
};
use create::create_post;

use crate::state::AppState;

pub mod create;

pub fn create_posts_router() -> Router<AppState> {
    Router::new().route("/", post(create_post))
}
