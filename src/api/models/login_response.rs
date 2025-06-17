use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct LoginResponse {
    pub token: String,
    pub token_type: String,
    pub expires_in_secs: u64,
}

impl LoginResponse {
    pub fn new(token: &str, token_type: &str, expires_in_secs: u64) -> Self {
        Self {
            token: token.to_string(),
            token_type: token_type.to_string(),
            expires_in_secs,
        }
    }
}

impl IntoResponse for LoginResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
