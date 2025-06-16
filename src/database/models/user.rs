use crate::database::models::Model;
use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Queryable, Selectable, AsChangeset, Serialize, Deserialize, Clone, PartialEq, Eq,
)]
#[diesel(table_name = crate::database::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub token_hash: String,
}

impl Model for User {
    type NewModel = NewUser;
    type PrimaryKeyType = i64;
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::database::schema::users)]
pub struct NewUser {
    pub token_hash: String,
}
