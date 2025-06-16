use crate::app::error::AppResult;
use crate::database::models::Model;
use crate::database::stores::user::UserStore;
use crate::database::Database;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::PooledConnection;
use std::sync::Arc;

mod user;

#[derive(Clone)]
pub struct Stores {
    pub user: Arc<UserStore>,
}

impl Stores {
    pub fn initialize(database: &Database) -> Self {
        Self {
            user: UserStore::initialize(database),
        }
    }
}

pub trait Store<M: Model> {
    fn initialize(database: &Database) -> Arc<Self>;
    fn get_database(&self) -> &Database;
    fn create(&self, new_entity: M::NewModel) -> AppResult<M>;
    fn find(&self, id: M::PrimaryKeyType) -> AppResult<Option<M>>;
    fn save(&self, entity: M) -> AppResult<M>;
    fn delete(&self, id: M::PrimaryKeyType) -> AppResult<Option<M>>;
    fn get_connection(&self) -> AppResult<PooledConnection<ConnectionManager<PgConnection>>> {
        Ok(self.get_database().get_connection()?)
    }
}
