use crate::app::services::user::UserService;
use crate::database::stores::Stores;
use std::sync::Arc;

mod user;

#[derive(Clone)]
pub struct Services {
    pub user: Arc<UserService>,
}

impl Services {
    pub fn initialize(stores: &Arc<Stores>) -> Arc<Self> {
        Arc::new(Self {
            user: UserService::initialize(stores),
        })
    }
}

pub trait Service {
    fn initialize(stores: &Arc<Stores>) -> Arc<Self>;
}
