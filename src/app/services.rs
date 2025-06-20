use crate::app::config::Config;
use crate::app::services::room::RoomService;
use crate::app::services::user::UserService;
use crate::database::stores::Stores;
use std::sync::Arc;

mod room;
mod user;

#[derive(Clone)]
pub struct Services {
    pub room: Arc<RoomService>,
    pub user: Arc<UserService>,
}

impl Services {
    pub fn initialize(config: &Arc<Config>, stores: &Arc<Stores>) -> Arc<Self> {
        Arc::new(Self {
            room: RoomService::initialize(config, stores),
            user: UserService::initialize(config, stores),
        })
    }
}

pub trait Service {
    fn initialize(config: &Arc<Config>, stores: &Arc<Stores>) -> Arc<Self>;
}
