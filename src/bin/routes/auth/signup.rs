use api_lib::{error::Error, runtime::Runtime};
use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginRequest {
    email: String,
    password: String,
}

/// # Signup
/// Authentication route for user signup,
pub async fn signup(
    State(_): State<Runtime>,
    Json(_): Json<LoginRequest>,
) -> Result<Json<Value>, Error> {
    Ok(Json(json!({})))
}
