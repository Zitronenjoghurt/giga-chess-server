use crate::app::error::AppError;
use crate::app::state::AppState;
use axum::{extract::FromRequestParts, http::request::Parts};

#[derive(Debug)]
pub struct LoginData {
    pub username: String,
    pub password: String,
}

impl FromRequestParts<AppState> for LoginData {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let headers = &parts.headers;

        let username = headers
            .get("X-Username")
            .and_then(|h| h.to_str().ok())
            .ok_or_else(|| AppError::invalid_input("Missing X-Username header"))?
            .to_string();

        let password = headers
            .get("X-Password")
            .and_then(|h| h.to_str().ok())
            .ok_or_else(|| AppError::invalid_input("Missing X-Password header"))?
            .to_string();

        Ok(LoginData { username, password })
    }
}
