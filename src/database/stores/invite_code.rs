use crate::app::config::Config;
use crate::app::error::AppResult;
use crate::database::models::invite_code::{InviteCode, NewInviteCode};
use crate::database::schema::invite_codes;
use crate::database::stores::Store;
use crate::database::Database;
use chrono::Utc;
use diesel::prelude::*;
use std::sync::Arc;
use uuid::Uuid;

pub struct InviteCodeStore {
    database: Arc<Database>,
}

impl Store<InviteCode> for InviteCodeStore {
    fn initialize(_config: &Arc<Config>, database: &Arc<Database>) -> Arc<Self> {
        Arc::new(Self {
            database: database.clone(),
        })
    }

    fn get_database(&self) -> &Arc<Database> {
        &self.database
    }

    async fn create(&self, new_entity: NewInviteCode) -> AppResult<InviteCode> {
        let mut conn = self.get_connection()?;
        let user = diesel::insert_into(invite_codes::table)
            .values(new_entity)
            .get_result(&mut conn)?;
        Ok(user)
    }

    async fn find(&self, id: Uuid) -> AppResult<Option<InviteCode>> {
        let mut connection = self.get_connection()?;
        let entity = invite_codes::table
            .find(id)
            .first::<InviteCode>(&mut connection)
            .optional()?;
        Ok(entity)
    }

    async fn save(&self, mut entity: InviteCode) -> AppResult<InviteCode> {
        let mut connection = self.get_connection()?;
        entity.updated_at = Utc::now();

        let updated_entity = diesel::update(invite_codes::table)
            .filter(invite_codes::id.eq(entity.id))
            .set(entity)
            .get_result::<InviteCode>(&mut connection)?;

        Ok(updated_entity)
    }

    async fn delete(&self, id: Uuid) -> AppResult<Option<InviteCode>> {
        let mut connection = self.get_connection()?;

        let entity = diesel::delete(invite_codes::table)
            .filter(invite_codes::id.eq(id))
            .get_result::<InviteCode>(&mut connection)
            .optional()?;

        Ok(entity)
    }
}
