use crate::app::config::Config;
use crate::app::services::Services;
use crate::database::stores::Stores;
use crate::database::Database;
use giga_chess::engine::Engine;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub database: Arc<Database>,
    pub engine: Arc<Engine>,
    pub services: Arc<Services>,
    pub stores: Arc<Stores>,
}

impl AppState {
    pub fn initialize(database_url: &str) -> Self {
        let database = Arc::new(Database::connect(database_url).unwrap());
        let stores = Stores::initialize(&database);
        let services = Services::initialize(&stores);

        Self {
            config: Config::initialize(),
            database,
            engine: Engine::initialize(),
            services,
            stores,
        }
    }
}
