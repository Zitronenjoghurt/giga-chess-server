use crate::database::models::room::NewRoom;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Validate, Serialize, Deserialize, ToSchema)]
pub struct RoomCreationBody {
    #[validate(length(min = 3, max = 50))]
    pub name: Option<String>,
    /// If the room is supposed to show up in the public room list.
    pub public: bool,
}

impl RoomCreationBody {
    pub fn get_new_room(&self, created_by: Uuid) -> NewRoom {
        NewRoom::new(self.name.clone(), self.public, created_by)
    }
}
