use axum::Router;
use posts::create_posts_router;

use crate::state::AppState;
pub mod posts;

pub fn create_main_router(state: AppState) -> Router {
    Router::new()
        .nest("/api/v1/posts", create_posts_router())
        .with_state(state)
        .layer(tower_http::trace::TraceLayer::new_for_http())
}
