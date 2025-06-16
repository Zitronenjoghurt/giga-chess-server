use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error")]
    DatabaseConnection(#[from] r2d2::Error),
    #[error("Database error")]
    DatabaseMigrationError(String),
    #[error("Database error")]
    DatabaseQuery(#[from] diesel::result::Error),
    #[error("Invalid username and/or token")]
    InvalidCredentials,
    #[error("Missing credentials: {0}")]
    MissingCredentials(String),
    #[error("An unexpected error occurred")]
    Argon2Hash(#[from] argon2::password_hash::Error),
}

impl AppError {
    pub fn get_status_code(&self) -> StatusCode {
        match self {
            AppError::InvalidCredentials | AppError::MissingCredentials(_) => {
                StatusCode::UNAUTHORIZED
            }
            Self::Argon2Hash(_)
            | Self::DatabaseConnection(_)
            | Self::DatabaseQuery(_)
            | Self::DatabaseMigrationError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.get_status_code();
        let message = self.to_string();
        (status, message).into_response()
    }
}
