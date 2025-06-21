use crate::app::config::Config;
use crate::app::error::AppResult;
use crate::database::models::session::{NewSession, Session};
use crate::database::schema::sessions;
use crate::database::stores::Store;
use crate::database::Database;
use async_trait::async_trait;
use chrono::Utc;
use diesel::prelude::*;
use std::sync::Arc;
use uuid::Uuid;

pub struct SessionStore {
    database: Arc<Database>,
}

#[async_trait]
impl Store<Session> for SessionStore {
    fn initialize(_config: &Arc<Config>, database: &Arc<Database>) -> Arc<Self> {
        Arc::new(Self {
            database: database.clone(),
        })
    }

    fn get_database(&self) -> &Arc<Database> {
        &self.database
    }

    async fn create(&self, new_entity: NewSession) -> AppResult<Session> {
        let mut conn = self.get_connection()?;
        let entity = diesel::insert_into(sessions::table)
            .values(new_entity)
            .get_result(&mut conn)?;
        Ok(entity)
    }

    async fn find(&self, id: Uuid) -> AppResult<Option<Session>> {
        let mut connection = self.get_connection()?;
        let entity = sessions::table
            .find(id)
            .first::<Session>(&mut connection)
            .optional()?;
        Ok(entity)
    }

    async fn save(&self, mut entity: Session) -> AppResult<Session> {
        let mut connection = self.get_connection()?;
        entity.updated_at = Utc::now();

        let updated_entity = diesel::update(sessions::table)
            .filter(sessions::id.eq(entity.id))
            .set(entity)
            .get_result::<Session>(&mut connection)?;

        Ok(updated_entity)
    }

    async fn delete(&self, id: Uuid) -> AppResult<Option<Session>> {
        let mut connection = self.get_connection()?;

        let entity = diesel::delete(sessions::table)
            .filter(sessions::id.eq(id))
            .get_result::<Session>(&mut connection)
            .optional()?;

        Ok(entity)
    }
}
