#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct MessageResponse {
    pub message: String,
}

impl MessageResponse {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

#[cfg(feature = "axum")]
impl axum::response::IntoResponse for MessageResponse {
    fn into_response(self) -> axum::response::Response {
        axum::Json(self).into_response()
    }
}
