use crate::app::error::AppError;
use crate::app::security::verify_jwt;
use crate::app::state::AppState;
use crate::database::models::user::User;
use crate::database::stores::Store;
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

        let auth_header = headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or(AppError::MissingCredentials(
                "Missing Authorization header".to_string(),
            ))?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or(AppError::MissingCredentials(
                "Invalid Authorization header format".to_string(),
            ))?;

        let claims = verify_jwt(token, &state.config.jwt_key)?;

        let Ok(uuid) = claims.get_uuid() else {
            return Err(AppError::InvalidCredentials);
        };

        let Some(user) = state.stores.user.find(uuid).await? else {
            return Err(AppError::InvalidCredentials);
        };

        Ok(AuthUser(user))
    }
}
