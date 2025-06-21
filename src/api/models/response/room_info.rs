use crate::api::models::response::user_info::PublicUserInfo;
use crate::database::models::room::Room;
use crate::database::models::user::User;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct PublicRoomInfo {
    pub uuid: String,
    pub name: Option<String>,
    pub white: Option<PublicUserInfo>,
    pub black: Option<PublicUserInfo>,
}

impl PublicRoomInfo {
    pub fn from_room_and_players(
        room: &Room,
        user_white: Option<&User>,
        user_black: Option<&User>,
    ) -> Self {
        let white = user_white.map(PublicUserInfo::from_user);
        let black = user_black.map(PublicUserInfo::from_user);

        Self {
            uuid: room.id.to_string(),
            name: room.name.clone(),
            white,
            black,
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct PrivateRoomInfo {
    pub uuid: String,
    pub name: Option<String>,
    pub public: bool,
    pub white: Option<PublicUserInfo>,
    pub black: Option<PublicUserInfo>,
}

impl PrivateRoomInfo {
    pub fn from_room_and_players(
        room: &Room,
        user_white: Option<&User>,
        user_black: Option<&User>,
    ) -> Self {
        let white = user_white.map(PublicUserInfo::from_user);
        let black = user_black.map(PublicUserInfo::from_user);

        Self {
            uuid: room.id.to_string(),
            name: room.name.clone(),
            public: room.public,
            white,
            black,
        }
    }
}

impl IntoResponse for PrivateRoomInfo {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
