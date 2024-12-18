use axum::Router;
use posts::create_posts_router;
mod posts;


pub fn create_main_router() -> Router {

    Router::new()
        .nest("/api/v1/posts", create_posts_router())
        .layer(tower_http::trace::TraceLayer::new_for_http())
}
