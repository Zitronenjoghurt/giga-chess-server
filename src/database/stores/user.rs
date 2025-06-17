use crate::app::error::AppResult;
use crate::database::models::user::{NewUser, User};
use crate::database::schema::users;
use crate::database::stores::Store;
use crate::database::Database;
use chrono::Utc;
use diesel::prelude::*;
use std::sync::Arc;
use uuid::Uuid;

pub struct UserStore {
    database: Arc<Database>,
}

impl UserStore {
    pub fn find_by_name(&self, name: &str) -> AppResult<Option<User>> {
        let mut connection = self.get_connection()?;
        let user = users::table
            .filter(users::name.eq(name))
            .first::<User>(&mut connection)
            .optional()?;
        Ok(user)
    }
}

impl Store<User> for UserStore {
    fn initialize(database: &Arc<Database>) -> Arc<Self> {
        Arc::new(Self {
            database: database.clone(),
        })
    }

    fn get_database(&self) -> &Arc<Database> {
        &self.database
    }

    fn create(&self, new_entity: NewUser) -> AppResult<User> {
        let mut conn = self.get_connection()?;
        let entity = diesel::insert_into(users::table)
            .values(new_entity)
            .get_result(&mut conn)?;
        Ok(entity)
    }

    fn find(&self, id: Uuid) -> AppResult<Option<User>> {
        let mut connection = self.get_connection()?;
        let entity = users::table
            .find(id)
            .first::<User>(&mut connection)
            .optional()?;
        Ok(entity)
    }

    fn save(&self, mut entity: User) -> AppResult<User> {
        let mut connection = self.get_connection()?;
        entity.updated_at = Utc::now();

        let updated_entity = diesel::update(users::table)
            .filter(users::id.eq(entity.id))
            .set(entity)
            .get_result::<User>(&mut connection)?;

        Ok(updated_entity)
    }

    fn delete(&self, id: Uuid) -> AppResult<Option<User>> {
        let mut connection = self.get_connection()?;

        let entity = diesel::delete(users::table)
            .filter(users::id.eq(id))
            .get_result::<User>(&mut connection)
            .optional()?;

        Ok(entity)
    }
}
