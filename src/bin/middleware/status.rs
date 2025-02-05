use std::time::Instant;

use api_lib::error::{Error, ServerError};
use axum::{
    body::{to_bytes, Body},
    http::{Request, Response},
    middleware::Next,
};
use chrono::Utc;
use serde_json::{json, Value};

/// Status Middleware, Adds the status block to the response body with the corresponding error message and code if there are any
pub async fn handle_status(req: Request<Body>, next: Next) -> Result<Response<Body>, Error> {
    let instant = Instant::now();

    // Retreive and unwrap the response of the next request
    let response = next.run(req).await;
    let (parts, body) = response.into_parts();

    let bytes = to_bytes(body, usize::MAX)
        .await
        .map_err(ServerError::from)?;

    // Unwrap the error message
    let message = match parts.status.is_client_error() || parts.status.is_server_error() {
        true => Some(String::from_utf8(bytes.to_vec()).map_err(ServerError::from)?),
        false => None,
    };

    // Unwrap the error code
    let code = match message.is_some() {
        true => Some(parts.status.as_u16()),
        false => None,
    };

    // If there is no error and the is data, parse the data to JSON object
    let data = match message.is_none() && !bytes.is_empty() {
        true => Some(serde_json::from_slice::<Value>(&bytes).map_err(ServerError::from)?),
        false => None,
    };

    let elapsed = instant.elapsed().as_millis();

    // Create the response JSON
    let result = serde_json::to_vec::<Value>(&json!({
        "data": data,
        "status": {
            "timestamp": Utc::now(),
            "elapsed": elapsed,
            "error_code": code,
            "error_message": message
        }
    }))
    .map_err(ServerError::from)?;

    Ok(Response::from_parts(parts, Body::from(result)))
}
