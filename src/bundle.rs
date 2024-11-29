use axum::{
    body::Body,
    extract::Path,
    http::{header, StatusCode},
    response::{IntoResponse, Response},
};
use include_dir::{include_dir, Dir};
use mime_guess::mime;
use time::Duration;

const ROOT: &str = "";

static FRONTEND_DIR: Dir<'_> = include_dir!("public");

pub async fn serve_asset(path: Option<Path<String>>) -> impl IntoResponse {
    let serve_file = |file_path: &str| {
        if let Some(file) = FRONTEND_DIR.get_file(file_path) {
            let mime_type = mime_guess::from_path(file_path).first_or_octet_stream();
            let cache_duration = if mime_type == mime::TEXT_HTML {
                Duration::ZERO
            } else {
                Duration::days(365)
            };

            Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, mime_type.to_string())
                .header(
                    header::CACHE_CONTROL,
                    format!("max-age={}", cache_duration.as_seconds_f32()),
                )
                .body(Body::from(file.contents().to_owned()))
                .unwrap()
        } else {
            Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from("File Not Found"))
                .unwrap()
        }
    };

    match path {
        Some(Path(path)) if path == ROOT => serve_file("index.html"),
        Some(Path(path)) => serve_file(&path),
        None => serve_file("index.html"),
    }
}
