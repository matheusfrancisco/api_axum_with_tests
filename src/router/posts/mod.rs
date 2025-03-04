use axum::{
    routing::{delete, get, patch, post},
    Router,
};
use create::create_post;

use crate::state::AppState;

pub mod create;
pub mod get_all_top_level;
pub mod get_one;
pub mod update;
pub mod delete;

pub fn create_posts_router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_post))
        .route("/", get(get(get_all_top_level::get_all_top_level)))
        .route("/:id", get(get_one::get_one_post))
        .route("/:id", patch(update::update_post_text))
        .route("/:id", delete(delete::delete_post))
}
