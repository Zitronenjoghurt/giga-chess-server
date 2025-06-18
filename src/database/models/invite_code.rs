use crate::database::models::Model;
use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    Debug,
    Identifiable,
    Queryable,
    Selectable,
    AsChangeset,
    Serialize,
    Deserialize,
    Clone,
    PartialEq,
    Eq,
)]
#[diesel(table_name = crate::database::schema::invite_codes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InviteCode {
    pub id: Uuid,
    pub used: bool,
    pub comment: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Model for InviteCode {
    type NewModel = NewInviteCode;
    type PrimaryKeyType = Uuid;
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::database::schema::invite_codes)]
pub struct NewInviteCode {
    pub id: Uuid,
    pub comment: Option<String>,
}

impl Default for NewInviteCode {
    fn default() -> Self {
        Self::new()
    }
}

impl NewInviteCode {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            comment: None,
        }
    }

    pub fn new_with_comment(comment: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            comment,
        }
    }
}
