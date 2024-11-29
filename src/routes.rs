use axum::extract::Path;
use axum::{extract::WebSocketUpgrade, http::StatusCode, response::Response, routing::get, Router};

use crate::bundle::serve_asset;
use crate::websocket::handle_socket;

#[inline]
pub fn svelte() -> Router {
    Router::new()
        .route(
            "/",
            get(|| async { serve_asset(Some(Path(String::from("")))).await }),
        )
        .route(
            "/*path",
            get(|path| async { serve_asset(Some(path)).await }),
        )
        .fallback_service(get(handler_404))
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
