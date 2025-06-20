use crate::database::models::user::User;
use crate::database::models::Model;
use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Associations, Identifiable, Insertable, Queryable, Selectable};
use giga_chess::prelude::Color;
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
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
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
    pub created_by: Uuid,
}

impl NewRoom {
    pub fn new(name: Option<String>, public: bool, created_by: Uuid, creator_color: Color) -> Self {
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
        }
    }
}
