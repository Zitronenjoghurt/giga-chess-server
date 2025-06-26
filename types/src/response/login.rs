#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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

#[cfg(feature = "axum")]
impl axum::response::IntoResponse for LoginResponse {
    fn into_response(self) -> axum::response::Response {
        axum::Json(self).into_response()
    }
}
