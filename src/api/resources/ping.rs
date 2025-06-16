use axum::response::{IntoResponse};
use axum::{routing::get, Router};
use crate::api::models::message_response::MessageResponse;
use crate::app::state::AppState;

/// Ping the API for a response.
///
/// This endpoint returns a simple pong message to indicate that the API is responsive.
#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "Pong", body = MessageResponse),
        (status = 500, description = "Server error"),
    ),
    tag = "Misc"
)]
async fn get_ping() -> impl IntoResponse {
    MessageResponse::new("Pong")
}

pub fn router() -> Router<AppState> {
    Router::<AppState>::new().route("/", get(get_ping))
}