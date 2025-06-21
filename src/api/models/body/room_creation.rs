use crate::database::models::room::NewRoom;
use giga_chess::prelude::Color;
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
    /// How much time (in microseconds) each player will get on their clock.
    #[validate(range(min = 1000000, max = i64::MAX))]
    pub time_micros: Option<i64>,
    /// By how much time (in microseconds) the player's clock will be incremented after their move.
    #[validate(range(min = 1000000, max = i64::MAX))]
    pub increment_micros: Option<i64>,
}

impl RoomCreationBody {
    pub fn get_new_room(&self, created_by: Uuid) -> NewRoom {
        NewRoom::new(
            self.name.clone(),
            self.public,
            created_by,
            Color::random(),
            self.time_micros,
            self.increment_micros,
        )
    }
}
