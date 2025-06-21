use crate::api::create_rate_limiter;
use crate::api::extractors::authentication::AuthUser;
use crate::api::models::body::uuid::UuidBody;
use crate::api::models::response::message::MessageResponse;
use crate::app::error::{AppError, AppResult};
use crate::app::state::AppState;
use crate::database::stores::Store;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use axum_valid::Valid;

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
    let Some(mut room) = state.stores.room.find(data.get_uuid()).await? else {
        return Err(AppError::not_found("Room"));
    };

    if room.created_by == user.id {
        return Err(AppError::bad_request("Cannot join your own room"));
    }

    if !room.public {
        return Err(AppError::not_found("Room"));
    }

    let success = room.join(&user);
    if !success {
        return Err(AppError::bad_request("Room is full"));
    }

    let room = state.stores.room.save(room).await?;
    let _ = state.services.session.start(&state.engine, &room).await?;
    Ok((StatusCode::OK, MessageResponse::new("Successfully joined")))
}

pub fn router() -> Router<AppState> {
    Router::new().route("/", post(post_room_join).layer(create_rate_limiter(5, 30)))
}
