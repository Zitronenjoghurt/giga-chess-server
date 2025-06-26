use crate::api::create_rate_limiter;
use crate::api::extractors::authentication::AuthUser;
use crate::app::error::AppResult;
use crate::app::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use axum_valid::Valid;
use giga_chess_api_types::body::uuid::UuidBody;
use giga_chess_api_types::response::message::MessageResponse;

/// Join a room.
#[utoipa::path(
    post,
    path = "/room/join",
    request_body = UuidBody,
    responses(
        (status = 200, description = "Successfully joined", body = MessageResponse),
        (status = 400, description = "Invalid body or room is full"),
        (status = 401, description = "Invalid credentials"),
        (status = 404, description = "Room not found"),
        (status = 429, description = "Too many requests"),
        (status = 500, description = "Server error"),
    ),
    tag = "Rooms",
    security(
        ("BearerAuth" = [])
    )
)]
async fn post_room_join(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    data: Valid<Json<UuidBody>>,
) -> AppResult<impl IntoResponse> {
    let room = state.services.room.join(data.get_uuid(), &user).await?;
    let _ = state.services.session.start(&state.engine, &room).await?;
    Ok((StatusCode::OK, MessageResponse::new("Successfully joined")))
}

pub fn router() -> Router<AppState> {
    Router::new().route("/", post(post_room_join).layer(create_rate_limiter(5, 30)))
}
