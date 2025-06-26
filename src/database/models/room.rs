use crate::database::models::user::User;
use crate::database::models::Model;
use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Associations, Identifiable, Insertable, Queryable, Selectable};
use giga_chess::prelude::Color;
use giga_chess_api_types::body::room_creation::RoomCreationBody;
use giga_chess_api_types::response::room_info::{PrivateRoomInfo, PublicRoomInfo};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    Debug,
    Identifiable,
    Queryable,
    Selectable,
    AsChangeset,
    Associations,
    Serialize,
    Deserialize,
    Clone,
    PartialEq,
    Eq,
)]
#[diesel(table_name = crate::database::schema::rooms)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(User, foreign_key = created_by))]
pub struct Room {
    pub id: Uuid,
    pub name: Option<String>,
    pub public: bool,
    pub player_white: Option<Uuid>,
    pub player_black: Option<Uuid>,
    pub time_micros: Option<i64>,
    pub increment_micros: Option<i64>,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Room {
    pub fn join(&mut self, user: &User) -> bool {
        if !self.can_join() {
            return false;
        }

        if self.player_white.is_none() {
            self.player_white = Some(user.id);
        } else {
            self.player_black = Some(user.id);
        }

        true
    }

    pub fn can_join(&self) -> bool {
        self.player_white.is_none() || self.player_black.is_none()
    }

    pub fn get_private_info(
        &self,
        user_white: Option<&User>,
        user_black: Option<&User>,
    ) -> PrivateRoomInfo {
        let white = user_white.map(User::get_public_info);
        let black = user_black.map(User::get_public_info);

        PrivateRoomInfo {
            uuid: self.id.to_string(),
            name: self.name.clone(),
            public: self.public,
            white,
            black,
        }
    }

    pub fn get_public_info(
        &self,
        user_white: Option<&User>,
        user_black: Option<&User>,
    ) -> PublicRoomInfo {
        let white = user_white.map(User::get_public_info);
        let black = user_black.map(User::get_public_info);

        PublicRoomInfo {
            uuid: self.id.to_string(),
            name: self.name.clone(),
            white,
            black,
        }
    }
}

impl Model for Room {
    type NewModel = NewRoom;
    type PrimaryKeyType = Uuid;
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::database::schema::rooms)]
pub struct NewRoom {
    pub id: Uuid,
    pub name: Option<String>,
    pub public: bool,
    pub player_white: Option<Uuid>,
    pub player_black: Option<Uuid>,
    pub time_micros: Option<i64>,
    pub increment_micros: Option<i64>,
    pub created_by: Uuid,
}

impl NewRoom {
    pub fn new(
        name: Option<String>,
        public: bool,
        created_by: Uuid,
        creator_color: Color,
        time_micros: Option<i64>,
        increment_micros: Option<i64>,
    ) -> Self {
        let player_white = match creator_color {
            Color::White => Some(created_by),
            Color::Black => None,
        };

        let player_black = match creator_color {
            Color::White => None,
            Color::Black => Some(created_by),
        };

        Self {
            id: Uuid::new_v4(),
            name,
            public,
            player_white,
            player_black,
            created_by,
            time_micros,
            increment_micros,
        }
    }

    pub fn from_creation_body(creation_body: RoomCreationBody, created_by: Uuid) -> Self {
        Self::new(
            creation_body.name.clone(),
            creation_body.public,
            created_by,
            Color::random(),
            creation_body.time_micros,
            creation_body.increment_micros,
        )
    }
}
