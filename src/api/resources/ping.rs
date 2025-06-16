use crate::api::extractors::authentication::AuthUser;
use crate::api::models::message_response::MessageResponse;
use crate::app::state::AppState;
use axum::response::IntoResponse;
use axum::{routing::get, Router};

/// Ping the API for a response.
///
/// This endpoint returns a simple pong message to indicate that the API is responsive.
#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "Pong", body = MessageResponse),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Server error"),
    ),
    tag = "Misc",
    security(
        ("UsernameAuth" = [], "TokenAuth" = [])
    )
)]
async fn get_ping(AuthUser(user): AuthUser) -> impl IntoResponse {
    MessageResponse::new(&format!("Hello, {}", user.name))
}

pub fn router() -> Router<AppState> {
    Router::<AppState>::new().route("/", get(get_ping))
}
