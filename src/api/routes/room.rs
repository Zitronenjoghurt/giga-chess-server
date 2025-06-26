use crate::api::create_rate_limiter;
use crate::api::extractors::authentication::AuthUser;
use crate::app::error::{AppError, AppResult};
use crate::app::state::AppState;
use crate::database::models::room::NewRoom;
use crate::database::stores::Store;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use axum_valid::Valid;
use giga_chess_api_types::body::room_creation::RoomCreationBody;
use giga_chess_api_types::query::pagination::PaginationQuery;
use giga_chess_api_types::response::room_info::PrivateRoomInfo;
use giga_chess_api_types::response::room_list::PublicRoomList;

pub mod join;

/// Get a list of public rooms.
#[utoipa::path(
    get,
    path = "/room",
    params(PaginationQuery),
    responses(
        (status = 200, description = "Successfully created", body = PublicRoomList),
        (status = 400, description = "Invalid query"),
        (status = 401, description = "Invalid credentials"),
        (status = 429, description = "Too many requests"),
        (status = 500, description = "Server error"),
    ),
    tag = "Rooms",
    security(
        ("BearerAuth" = [])
    )
)]
async fn get_room(
    AuthUser(_): AuthUser,
    State(state): State<AppState>,
    Query(pagination_query): Query<PaginationQuery>,
) -> AppResult<impl IntoResponse> {
    let list = state
        .services
        .room
        .public_room_list(pagination_query)
        .await?;
    Ok((StatusCode::OK, list))
}

/// Create a new room.
///
/// People will be able to join to start a session
#[utoipa::path(
    post,
    path = "/room",
    request_body = RoomCreationBody,
    responses(
        (status = 201, description = "Successfully created", body = PrivateRoomInfo),
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
    Valid(Json(data)): Valid<Json<RoomCreationBody>>,
) -> AppResult<impl IntoResponse> {
    let user_rooms = state.stores.room.find_by_user(&user).await?;
    if user_rooms.len() >= state.config.room_creation_limit {
        return Err(AppError::bad_request(&format!(
            "Room limit reached ({})",
            state.config.room_creation_limit
        )));
    };

    let new_room = NewRoom::from_creation_body(data, user.id);
    let room = state.stores.room.create(new_room).await?;

    let user_white = if room.player_white.is_some() {
        Some(&user)
    } else {
        None
    };

    let user_black = if room.player_black.is_some() {
        Some(&user)
    } else {
        None
    };

    let info = room.get_private_info(user_white, user_black);
    Ok((StatusCode::CREATED, info))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_room).layer(create_rate_limiter(5, 30)))
        .route("/", post(post_room).layer(create_rate_limiter(5, 30)))
        .nest("/join", join::router())
}
