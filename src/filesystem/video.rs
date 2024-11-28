use axum::http::{header, HeaderMap, StatusCode};
use axum::{extract::Query, response::Response, routing::get, Router};
use std::collections::HashMap;
use std::io::SeekFrom;
use std::path::Path;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncSeekExt};

use super::escape_path;

#[inline]
pub fn video_router() -> Router {
    Router::new().route("/video", get(video_handler))
}

fn extract_extension(filename: &str) -> Result<String, String> {
    Path::new(filename)
        .extension()
        .and_then(|ext| ext.to_str())
        .map(String::from)
        .ok_or_else(|| "No valid file extension found.".to_string())
}

fn get_mime_type(extension: &str) -> &str {
    match extension {
        "mp4" => "video/mp4",
        "webm" => "video/webm",
        "ogg" => "video/ogg",
        "mov" => "video/quicktime",
        _ => "application/octet-stream",
    }
}

async fn create_body(file: &mut File, start: u64, end: u64) -> Result<Vec<u8>, StatusCode> {
    file.seek(SeekFrom::Start(start))
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let mut buffer = vec![0; (end - start + 1) as usize];
    file.read_exact(&mut buffer)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(buffer)
}

pub async fn video_handler(
    Query(params): Query<HashMap<String, String>>,
    headers: HeaderMap,
) -> Result<Response, StatusCode> {
    let query = params.get("query").ok_or(StatusCode::BAD_REQUEST)?;
    let file_path = escape_path(query);

    let mut file = File::open(&file_path)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    let metadata = file
        .metadata()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let file_size = metadata.len();

    let extension = extract_extension(query).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    /* Range requests */
    if let Some(range_header) = headers.get(header::RANGE) {
        let range = range_header.to_str().unwrap_or("").replace("bytes=", "");
        let (start, end) = match range.split_once('-') {
            Some((s, e)) => (
                s.parse::<u64>().unwrap_or(0),
                e.parse::<u64>().ok().unwrap_or(file_size - 1),
            ),
            None => (0, file_size - 1),
        };

        if start >= file_size || end >= file_size || start > end {
            return Err(StatusCode::RANGE_NOT_SATISFIABLE);
        }

        let buffer = create_body(&mut file, start, end).await?;
        return Ok(Response::builder()
            .status(StatusCode::PARTIAL_CONTENT)
            .header(header::CONTENT_TYPE, get_mime_type(&extension))
            .header(
                header::CONTENT_RANGE,
                format!("bytes {}-{}/{}", start, end, file_size),
            )
            .header(header::CONTENT_LENGTH, buffer.len().to_string())
            .body(buffer.into())
            .unwrap());
    }

    /* Serve the entire file if no Range header */
    let buffer = create_body(&mut file, 0, file_size - 1).await?;
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, get_mime_type(&extension))
        .header(header::CONTENT_LENGTH, file_size.to_string())
        .header(header::ACCEPT_RANGES, "bytes")
        .body(buffer.into())
        .unwrap())
}
