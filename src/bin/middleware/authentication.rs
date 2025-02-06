use api_lib::error::Error;
use api_lib::models::Identity;
use api_lib::utils::cryptography::sha256::cipher;
use axum::{
    body::Body,
    http::{Request, Response},
    middleware::Next,
};

/// Authentication Middleware, intended for API - KEY header
pub async fn handle_authentication(
    mut req: Request<Body>,
    next: Next,
) -> Result<Response<Body>, Error> {
    let secret_sha256 = "9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08"; // "test"
    if let Some(auth_header) = req.headers().get("X-API_KEY") {
        let secret = auth_header.to_str().unwrap_or("");

        // XXX:
        // based on "secret", get a valid secret token from the database?
        // some protection against timing attacks?
        // some protection for the token data -> token is checked with the database
        // based on that token, get the sha-256 of secret token, for its pair in the DB & user basic data
        // on not found, exit with 401 Unauthorized

        let sha256 = cipher(secret);
        tracing::info!("SHA256 of a secret: {}", sha256);
        if sha256 == secret_sha256 {
            // Token is valid, proceed to the handler, get some basic user data
            let user = Identity::default();
            req.extensions_mut().insert(user);
        } else {
            // Return 401 Unauthorized if the API key is invalid
            return Err(Error::Unauthorized);
        }
    } else {
        // Return 401 Unauthorized if no API key is provided
        return Err(Error::Unauthorized);
    }

    // Retrieve the response of the next request
    let response = next.run(req).await;
    // Deconstruct the response in its parts and body
    let (parts, body) = response.into_parts();
    Ok(Response::from_parts(parts, body))
}
