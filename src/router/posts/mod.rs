use axum::{
    routing::{delete, get, patch, post},
    Router,
};
use create::create_post;

pub mod create;
pub mod get_all_top_level;


use crate::state::AppState;


pub fn create_posts_router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_post))
        .route("/", get(get_all_top_level::get_all_top_level))
}
