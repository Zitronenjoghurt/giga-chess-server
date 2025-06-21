use crate::api::create_rate_limiter;
use crate::api::models::body::login::LoginBody;
use crate::api::models::response::login::LoginResponse;
use crate::app::error::AppResult;
use crate::app::state::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use axum_valid::Valid;

const TOKEN_TTL_SECS: u64 = 60 * 60 * 24 * 7;

/// Login to your account.
///
/// This will create an authentication token that needs to be provided in the Authorization header.
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
    let jwt = state
        .services
        .user
        .login(&data.username, &data.password)
        .await?;
    Ok(LoginResponse::new(&jwt, "Bearer", TOKEN_TTL_SECS))
}

pub fn router() -> Router<AppState> {
    Router::<AppState>::new()
        .route("/", post(post_login))
        .layer(create_rate_limiter(5, 30))
}
