use crate::api::create_rate_limiter;
use crate::app::error::AppResult;
use crate::app::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use axum_valid::Valid;
use giga_chess_api_types::body::register::RegisterBody;
use giga_chess_api_types::response::message::MessageResponse;

/// Register a new account.
///
/// A valid invite code will have to be provided. Administrators can generate invite codes through the CLI.
#[utoipa::path(
    post,
    path = "/register",
    request_body = RegisterBody,
    responses(
        (status = 201, description = "Successfully registered", body = MessageResponse),
        (status = 400, description = "Invalid body"),
        (status = 409, description = "Username already exists"),
        (status = 429, description = "Too many requests"),
        (status = 500, description = "Server error"),
    ),
    tag = "Auth"
)]
async fn post_register(
    State(state): State<AppState>,
    data: Valid<Json<RegisterBody>>,
) -> AppResult<impl IntoResponse> {
    let _ = state
        .services
        .user
        .register(&data.invite_code, &data.username, &data.password)
        .await?;

    Ok((
        StatusCode::CREATED,
        MessageResponse::new("Successfully registered"),
    ))
}

pub fn router() -> Router<AppState> {
    Router::<AppState>::new()
        .route("/", post(post_register))
        .layer(create_rate_limiter(5, 30))
}
