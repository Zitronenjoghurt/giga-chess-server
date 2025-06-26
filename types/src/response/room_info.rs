use crate::response::user_info::PublicUserInfo;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct PublicRoomInfo {
    pub uuid: String,
    pub name: Option<String>,
    pub white: Option<PublicUserInfo>,
    pub black: Option<PublicUserInfo>,
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct PrivateRoomInfo {
    pub uuid: String,
    pub name: Option<String>,
    pub public: bool,
    pub white: Option<PublicUserInfo>,
    pub black: Option<PublicUserInfo>,
}

#[cfg(feature = "axum")]
impl axum::response::IntoResponse for PrivateRoomInfo {
    fn into_response(self) -> axum::response::Response {
        axum::Json(self).into_response()
    }
}
