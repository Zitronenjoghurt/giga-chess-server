use crate::database::models::room::Room;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct RoomCreationResponse {
    pub id: String,
    pub public: bool,
}

impl RoomCreationResponse {
    pub fn new(id: Uuid, public: bool) -> Self {
        Self {
            id: id.to_string(),
            public,
        }
    }

    pub fn from_room(room: &Room) -> Self {
        Self::new(room.id, room.public)
    }
}

impl IntoResponse for RoomCreationResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
