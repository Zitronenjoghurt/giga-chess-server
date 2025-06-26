use crate::general::pagination::Pagination;
use crate::response::room_info::PublicRoomInfo;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct PublicRoomList {
    pub rooms: Vec<PublicRoomInfo>,
    pub pagination: Pagination,
}

#[cfg(feature = "axum")]
impl axum::response::IntoResponse for PublicRoomList {
    fn into_response(self) -> axum::response::Response {
        axum::Json(self).into_response()
    }
}
