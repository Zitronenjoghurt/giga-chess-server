use crate::api::models::general::pagination::Pagination;
use crate::api::models::response::room_info::PublicRoomInfo;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct PublicRoomList {
    pub rooms: Vec<PublicRoomInfo>,
    pub pagination: Pagination,
}

impl IntoResponse for PublicRoomList {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
