use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct MessageResponse {
    pub message: String,
}

impl MessageResponse {
    pub fn new(message: &str) -> Self {
        Self { message: message.to_string() }
    }
}

impl IntoResponse for MessageResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}