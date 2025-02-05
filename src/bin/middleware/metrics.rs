use std::time::Instant;

use api_lib::error::Error;
use axum::{
    body::Body,
    extract::MatchedPath,
    http::{Request, Response},
    middleware::Next,
};

pub async fn handle_metrics(req: Request<Body>, next: Next) -> Result<Response<Body>, Error> {
    let start = Instant::now();
    let path = if let Some(matched_path) = req.extensions().get::<MatchedPath>() {
        matched_path.as_str().to_owned()
    } else {
        req.uri().path().to_owned()
    };
    let method = req.method().clone();
    let response = next.run(req).await;
    let status = response.status().as_u16().to_string();
    let labels = [
        ("method", method.to_string()),
        ("path", path),
        ("status", status),
    ];

    metrics::counter!("http_requests_total", &labels).increment(1);

    let latency = start.elapsed().as_secs_f64();
    metrics::histogram!("http_requests_duration_seconds", &labels).record(latency);

    Ok(response)
}
