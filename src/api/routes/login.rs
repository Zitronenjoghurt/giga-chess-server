use crate::api::create_rate_limiter;
use crate::api::models::body::login::LoginBody;
use crate::api::models::response::login::LoginResponse;
use crate::app::error::{AppError, AppResult};
use crate::app::security::{generate_jwt, verify_bytes};
use crate::app::state::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use axum_valid::Valid;
use std::time::Duration;

const TOKEN_TTL_SECS: u64 = 60 * 60 * 24 * 7;

/// Log-in to your account and receive an authentication token.
#[utoipa::path(
    post,
    path = "/login",
    request_body = LoginBody,
    responses(
        (status = 200, description = "Successfully logged in", body = LoginResponse),
        (status = 400, description = "Invalid body"),
        (status = 401, description = "Invalid credentials"),
        (status = 429, description = "Too many requests"),
        (status = 500, description = "Server error"),
    ),
    tag = "Auth"
)]
async fn post_login(
    State(state): State<AppState>,
    data: Valid<Json<LoginBody>>,
) -> AppResult<impl IntoResponse> {
    let Some(user) = state.stores.user.find_by_name(&data.username)? else {
        return Err(AppError::InvalidCredentials);
    };

    if !verify_bytes(data.password.as_bytes(), &user.password_hash)? {
        return Err(AppError::InvalidCredentials);
    }

    let Ok(jwt) = generate_jwt(
        &user,
        &state.config.jwt_key,
        Duration::from_secs(TOKEN_TTL_SECS),
    ) else {
        return Err(AppError::InvalidCredentials);
    };

    Ok(LoginResponse::new(&jwt, "Bearer", TOKEN_TTL_SECS))
}

pub fn router() -> Router<AppState> {
    Router::<AppState>::new()
        .route("/", post(post_login))
        .layer(create_rate_limiter(5, 30))
}
