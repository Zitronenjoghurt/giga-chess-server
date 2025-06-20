use crate::api::models::response::user_info::PublicUserInfo;
use crate::database::models::room::Room;
use crate::database::models::user::User;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct PublicRoomInfo {
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
            name: room.name.clone(),
            white,
            black,
        }
    }
}
