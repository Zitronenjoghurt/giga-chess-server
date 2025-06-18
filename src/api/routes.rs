use crate::app::state::AppState;
use axum::Router;

pub mod login;
pub mod ping;
pub mod register;
pub mod room;

pub fn api_routes() -> Router<AppState> {
    Router::<AppState>::new()
        .nest("/login", login::router())
        .nest("/ping", ping::router())
        .nest("/register", register::router())
        .nest("/room", room::router())
}
