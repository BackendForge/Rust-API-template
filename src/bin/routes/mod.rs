use axum::{
    http::{StatusCode, Uri},
    response::IntoResponse,
};

pub mod auth;

/// 404 Not found fallback route
pub async fn handler_404(uri: Uri) -> impl IntoResponse {
    log::error!("{uri}");

    (StatusCode::NOT_FOUND, "Not Found")
}
