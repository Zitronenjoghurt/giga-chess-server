use crate::app::config::Config;
use crate::app::error::AppResult;
use crate::database::models::Model;
use crate::database::stores::invite_code::InviteCodeStore;
use crate::database::stores::room::RoomStore;
use crate::database::stores::user::UserStore;
use crate::database::Database;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::PooledConnection;
use std::sync::Arc;

pub mod invite_code;
pub mod room;
pub mod user;

#[derive(Clone)]
pub struct Stores {
    pub invite_code: Arc<InviteCodeStore>,
    pub room: Arc<RoomStore>,
    pub user: Arc<UserStore>,
}

impl Stores {
    pub fn initialize(config: &Arc<Config>, database: &Arc<Database>) -> Arc<Self> {
        Arc::new(Self {
            invite_code: InviteCodeStore::initialize(config, database),
            room: RoomStore::initialize(config, database),
            user: UserStore::initialize(config, database),
        })
    }
}

pub trait Store<M: Model> {
    fn initialize(config: &Arc<Config>, database: &Arc<Database>) -> Arc<Self>;
    fn get_database(&self) -> &Arc<Database>;
    async fn create(&self, new_entity: M::NewModel) -> AppResult<M>;
    async fn find(&self, id: M::PrimaryKeyType) -> AppResult<Option<M>>;
    async fn save(&self, entity: M) -> AppResult<M>;
    async fn delete(&self, id: M::PrimaryKeyType) -> AppResult<Option<M>>;
    fn get_connection(&self) -> AppResult<PooledConnection<ConnectionManager<PgConnection>>> {
        Ok(self.get_database().get_connection()?)
    }
}
