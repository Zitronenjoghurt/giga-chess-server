mod integration;
mod stores;

use crate::app::build_app;
use crate::app::state::AppState;
use axum_test::TestServer;
use std::env;

pub fn build_test_app_state() -> AppState {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let state = AppState::initialize(&database_url);
    state.database.clear().unwrap();
    state
}

pub fn build_test_server() -> (TestServer, AppState) {
    let state = build_test_app_state();
    let app = build_app(state.clone());
    (TestServer::new(app).unwrap(), state)
}
