#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct PublicUserInfo {
    pub name: String,
}

#[cfg(feature = "axum")]
impl axum::response::IntoResponse for PublicUserInfo {
    fn into_response(self) -> axum::response::Response {
        axum::Json(self).into_response()
    }
}
