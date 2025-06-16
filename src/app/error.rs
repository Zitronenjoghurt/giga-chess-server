use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Authorization error: {0}")]
    Authorization(String),
    #[error("Database connection error")]
    DatabaseConnection(#[from] r2d2::Error),
    #[error("Database migration error")]
    DatabaseMigrationError(String),
    #[error("Database query error")]
    DatabaseQuery(#[from] diesel::result::Error),
}

impl AppError {
    pub fn get_status_code(&self) -> StatusCode {
        match self {
            Self::Authorization(_) => StatusCode::UNAUTHORIZED,
            Self::DatabaseConnection(_)
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
