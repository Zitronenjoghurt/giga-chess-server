use crate::database::models::Model;
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = crate::database::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i64,
    token_hash: String,
}

impl Model for User {
    type NewModel = NewUser;
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::database::schema::users)]
pub struct NewUser {
    pub token_hash: String,
}
