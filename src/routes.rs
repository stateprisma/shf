use axum::{handler::HandlerWithoutStateExt, http::StatusCode, Router};
use tower_http::services::ServeDir;

#[inline]
pub fn svelte(build_dir: &str) -> Router {
    Router::new()
        .fallback_service(ServeDir::new(build_dir).not_found_service(handler_404.into_service()))
}

#[inline]
async fn handler_404() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Page not found")
}
