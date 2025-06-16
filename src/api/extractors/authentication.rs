use crate::app::error::AppError;
use crate::app::security::verify_bytes;
use crate::app::state::AppState;
use crate::database::models::user::User;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;

pub struct AuthUser(pub User);

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let headers = &parts.headers;

        let username = headers
            .get("X-Username")
            .and_then(|v| v.to_str().ok())
            .ok_or(AppError::MissingCredentials(
                "Missing X-Username header".to_string(),
            ))?;

        let token = headers.get("X-Token").and_then(|v| v.to_str().ok()).ok_or(
            AppError::MissingCredentials("Missing X-Token header".to_string()),
        )?;

        let user = state.stores.user.find_by_name(username)?;

        let Some(user) = user else {
            return Err(AppError::InvalidCredentials);
        };

        let is_token_valid = verify_bytes(token.as_bytes(), &user.token_hash)?;

        if !is_token_valid {
            Err(AppError::InvalidCredentials)
        } else {
            Ok(AuthUser(user))
        }
    }
}
