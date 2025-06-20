use crate::app::config::Config;
use crate::app::error::AppResult;
use crate::database::models::user::{NewUser, User};
use crate::database::schema::users;
use crate::database::stores::Store;
use crate::database::Database;
use chrono::Utc;
use diesel::prelude::*;
use moka::future::Cache;
use std::sync::Arc;
use std::time::Duration;
use uuid::Uuid;

pub struct UserStore {
    database: Arc<Database>,
    find_cache: Cache<Uuid, User>,
}

impl UserStore {
    pub async fn find_by_name(&self, name: &str) -> AppResult<Option<User>> {
        let mut connection = self.get_connection()?;
        let user = users::table
            .filter(users::name.eq(name))
            .first::<User>(&mut connection)
            .optional()?;
        Ok(user)
    }
}

impl Store<User> for UserStore {
    fn initialize(config: &Arc<Config>, database: &Arc<Database>) -> Arc<Self> {
        let find_cache = Cache::builder()
            .max_capacity(config.user_cache_capacity)
            .time_to_live(Duration::from_secs(config.user_cache_ttl_sec))
            .build();

        Arc::new(Self {
            database: database.clone(),
            find_cache,
        })
    }

    fn get_database(&self) -> &Arc<Database> {
        &self.database
    }

    async fn create(&self, new_entity: NewUser) -> AppResult<User> {
        let mut conn = self.get_connection()?;
        let entity = diesel::insert_into(users::table)
            .values(new_entity)
            .get_result(&mut conn)?;
        Ok(entity)
    }

    async fn find(&self, id: Uuid) -> AppResult<Option<User>> {
        if let Some(entity) = self.find_cache.get(&id).await {
            return Ok(Some(entity));
        };

        let mut connection = self.get_connection()?;
        let entity = users::table
            .find(id)
            .first::<User>(&mut connection)
            .optional()?;

        if let Some(entity) = entity.clone() {
            self.find_cache.insert(id, entity).await;
        }

        Ok(entity)
    }

    async fn save(&self, mut entity: User) -> AppResult<User> {
        let mut connection = self.get_connection()?;
        entity.updated_at = Utc::now();

        let updated_entity = diesel::update(users::table)
            .filter(users::id.eq(entity.id))
            .set(entity)
            .get_result::<User>(&mut connection)?;

        self.find_cache
            .insert(updated_entity.id, updated_entity.clone())
            .await;

        Ok(updated_entity)
    }

    async fn delete(&self, id: Uuid) -> AppResult<Option<User>> {
        let mut connection = self.get_connection()?;

        let entity = diesel::delete(users::table)
            .filter(users::id.eq(id))
            .get_result::<User>(&mut connection)
            .optional()?;

        self.find_cache.invalidate(&id).await;

        Ok(entity)
    }
}
