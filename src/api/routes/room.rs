use crate::api::create_rate_limiter;
use crate::api::extractors::authentication::AuthUser;
use crate::api::models::body::room_creation::RoomCreationBody;
use crate::api::models::response::room_creation::RoomCreationResponse;
use crate::app::error::{AppError, AppResult};
use crate::app::state::AppState;
use crate::database::stores::Store;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use axum_valid::Valid;

/// Create a new room that people will be able to join to start a session.
#[utoipa::path(
    post,
    path = "/room",
    request_body = RoomCreationBody,
    responses(
        (status = 201, description = "Successfully created", body = RoomCreationResponse),
        (status = 400, description = "Invalid body or room limit reached"),
        (status = 401, description = "Invalid credentials"),
        (status = 429, description = "Too many requests"),
        (status = 500, description = "Server error"),
    ),
    tag = "Rooms",
    security(
        ("BearerAuth" = [])
    )
)]
async fn post_room(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    data: Valid<Json<RoomCreationBody>>,
) -> AppResult<impl IntoResponse> {
    let user_rooms = state.stores.room.find_by_user(&user)?;
    if user_rooms.len() >= state.config.room_creation_limit {
        return Err(AppError::bad_request(&format!(
            "Room limit reached ({})",
            state.config.room_creation_limit
        )));
    };

    let new_room = data.get_new_room(user.id);
    let room = state.stores.room.create(new_room)?;

    Ok((StatusCode::CREATED, RoomCreationResponse::from_room(&room)))
}

pub fn router() -> Router<AppState> {
    Router::new().route("/", post(post_room).layer(create_rate_limiter(5, 30)))
}
