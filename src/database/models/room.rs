use crate::database::models::user::User;
use crate::database::models::Model;
use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Associations, Identifiable, Insertable, Queryable, Selectable};
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
    pub created_by: Uuid,
}

impl NewRoom {
    pub fn new(name: Option<String>, public: bool, created_by: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            public,
            created_by,
        }
    }
}
