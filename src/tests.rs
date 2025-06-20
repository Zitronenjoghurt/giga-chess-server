mod integration;
mod stores;

use crate::app::build_app;
use crate::app::state::AppState;
use crate::database::models::invite_code::NewInviteCode;
use crate::database::stores::Store;
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

pub async fn build_test_server_with_user(
    username: &str,
    password: &str,
) -> (TestServer, AppState, String) {
    let (server, state) = build_test_server();

    let invite_code = state
        .stores
        .invite_code
        .create(NewInviteCode::new())
        .await
        .unwrap();

    let _ = state
        .services
        .user
        .register(&invite_code.id.to_string(), username, password)
        .await
        .unwrap();

    let jwt = state.services.user.login(username, password).await.unwrap();

    (server, state, jwt)
}
