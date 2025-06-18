use crate::app::error::AppResult;
use crate::database::models::room::{NewRoom, Room};
use crate::database::models::user::User;
use crate::database::schema::rooms;
use crate::database::stores::Store;
use crate::database::Database;
use chrono::Utc;
use diesel::prelude::*;
use std::sync::Arc;
use uuid::Uuid;

pub struct RoomStore {
    database: Arc<Database>,
}

impl RoomStore {
    pub fn find_by_user(&self, user: &User) -> AppResult<Vec<Room>> {
        let mut connection = self.get_connection()?;
        Ok(Room::belonging_to(&user).load::<Room>(&mut connection)?)
    }
}

impl Store<Room> for RoomStore {
    fn initialize(database: &Arc<Database>) -> Arc<Self> {
        Arc::new(Self {
            database: database.clone(),
        })
    }

    fn get_database(&self) -> &Arc<Database> {
        &self.database
    }

    fn create(&self, new_entity: NewRoom) -> AppResult<Room> {
        let mut conn = self.get_connection()?;
        let entity = diesel::insert_into(rooms::table)
            .values(new_entity)
            .get_result(&mut conn)?;
        Ok(entity)
    }

    fn find(&self, id: Uuid) -> AppResult<Option<Room>> {
        let mut connection = self.get_connection()?;
        let entity = rooms::table
            .find(id)
            .first::<Room>(&mut connection)
            .optional()?;
        Ok(entity)
    }

    fn save(&self, mut entity: Room) -> AppResult<Room> {
        let mut connection = self.get_connection()?;
        entity.updated_at = Utc::now();

        let updated_entity = diesel::update(rooms::table)
            .filter(rooms::id.eq(entity.id))
            .set(entity)
            .get_result::<Room>(&mut connection)?;

        Ok(updated_entity)
    }

    fn delete(&self, id: Uuid) -> AppResult<Option<Room>> {
        let mut connection = self.get_connection()?;

        let entity = diesel::delete(rooms::table)
            .filter(rooms::id.eq(id))
            .get_result::<Room>(&mut connection)
            .optional()?;

        Ok(entity)
    }
}
