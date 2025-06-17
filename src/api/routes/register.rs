use crate::api::create_rate_limiter;
use crate::api::extractors::register_data::RegisterData;
use crate::api::models::message_response::MessageResponse;
use crate::app::error::AppResult;
use crate::app::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::put;
use axum::Router;

/// Register a new account by using a valid invite code.
#[utoipa::path(
    put,
    path = "/register",
    responses(
        (status = 201, description = "Successfully registered", body = MessageResponse),
        (status = 400, description = "Invalid invite code or missing headers"),
        (status = 409, description = "Username already exists"),
        (status = 500, description = "Server error"),
    ),
    params(
        ("X-Invite-Code" = String, Header, description = "Valid invite code for registration"),
        ("X-Username" = String, Header, description = "Desired username for the new account"),
        ("X-Password" = String, Header, description = "Password for the new account")
    ),
    tag = "Auth"
)]
async fn put_register(
    State(state): State<AppState>,
    data: RegisterData,
) -> AppResult<impl IntoResponse> {
    let _ = state
        .services
        .user
        .register(&data.invite_code, &data.username, &data.password)?;

    Ok((
        StatusCode::CREATED,
        MessageResponse::new("Successfully registered"),
    ))
}

pub fn router() -> Router<AppState> {
    Router::<AppState>::new()
        .route("/register", put(put_register))
        .layer(create_rate_limiter(5, 60))
}
