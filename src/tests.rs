mod stores;

use crate::app::state::AppState;
use std::env;

pub fn build_test_app_state() -> AppState {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let state = AppState::initialize(&database_url);
    state.database.clear().unwrap();
    state
}
