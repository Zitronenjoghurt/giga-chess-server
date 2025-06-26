use crate::database::models::Model;
use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use giga_chess_api_types::response::user_info::PublicUserInfo;
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
#[diesel(table_name = crate::database::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub invite_code_id: Uuid,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn get_public_info(&self) -> PublicUserInfo {
        PublicUserInfo {
            name: self.name.clone(),
        }
    }
}

impl Model for User {
    type NewModel = NewUser;
    type PrimaryKeyType = Uuid;
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::database::schema::users)]
pub struct NewUser {
    id: Uuid,
    name: String,
    invite_code_id: Uuid,
    password_hash: String,
}

impl NewUser {
    pub fn new(name: &str, invite_code_id: Uuid, password_hash: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_lowercase(),
            invite_code_id,
            password_hash: password_hash.to_string(),
        }
    }
}
