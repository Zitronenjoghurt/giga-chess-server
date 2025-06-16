use crate::app::error::AppResult;
use crate::database::models::user::{NewUser, User};
use crate::database::schema::users;
use crate::database::stores::Store;
use crate::database::Database;
use diesel::RunQueryDsl;
use std::sync::Arc;

pub struct UserStore {
    database: Database,
}

impl Store<User> for UserStore {
    fn initialize(database: &Database) -> Arc<Self> {
        Arc::new(Self {
            database: database.clone(),
        })
    }

    fn get_database(&self) -> &Database {
        &self.database
    }

    fn create(&self, new_model: NewUser) -> AppResult<User> {
        let mut conn = self.get_connection()?;
        let user = diesel::insert_into(users::table)
            .values(new_model)
            .get_result(&mut conn)?;
        Ok(user)
    }
}
