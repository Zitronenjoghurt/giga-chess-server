use crate::database::stores::Stores;
use crate::database::Database;
use giga_chess::engine::Engine;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub database: Database,
    pub engine: Arc<Engine>,
    pub stores: Stores,
}

impl AppState {
    pub fn initialize(database_url: &str) -> Self {
        let database = Database::connect(database_url).unwrap();
        let stores = Stores::initialize(&database);

        Self {
            database,
            engine: Engine::initialize(),
            stores,
        }
    }
}
