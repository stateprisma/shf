use axum::{
    extract::WebSocketUpgrade, handler::HandlerWithoutStateExt, http::StatusCode,
    response::Response, routing::get, Router,
};

use tower_http::services::ServeDir;

use crate::websocket::handle_socket;

#[inline]
pub fn svelte(build_dir: &str) -> Router {
    Router::new()
        .fallback_service(ServeDir::new(build_dir).not_found_service(handler_404.into_service()))
}

#[inline]
async fn handler_404() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Page not found")
}

#[inline]
pub fn websocket() -> Router {
    Router::new().route("/connect", get(ws_handler))
}

#[inline]
async fn ws_handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}
