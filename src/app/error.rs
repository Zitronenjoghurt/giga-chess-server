use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;
use tracing::error;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("{subject} already exists")]
    AlreadyExists { subject: String },
    #[error("An unexpected error occurred")]
    Argon2Hash(#[from] argon2::password_hash::Error),
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Database error")]
    DatabaseConnection(#[from] r2d2::Error),
    #[error("Database error")]
    DatabaseMigrationError(String),
    #[error("Database error")]
    DatabaseQuery(#[from] diesel::result::Error),
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Invalid credentials")]
    JWT(#[from] jsonwebtoken::errors::Error),
    #[error("Missing credentials: {0}")]
    MissingCredentials(String),
    #[error("{subject} not found")]
    NotFound { subject: String },
    #[error("Serialization error")]
    Serialization(String),
}

impl AppError {
    pub fn get_status_code(&self) -> StatusCode {
        match self {
            Self::AlreadyExists { .. } => StatusCode::CONFLICT,
            Self::BadRequest(_) | Self::InvalidInput(_) => StatusCode::BAD_REQUEST,
            Self::InvalidCredentials | Self::MissingCredentials(_) | Self::JWT(_) => {
                StatusCode::UNAUTHORIZED
            }
            Self::Argon2Hash(_)
            | Self::DatabaseConnection(_)
            | Self::DatabaseQuery(_)
            | Self::DatabaseMigrationError(_)
            | Self::Serialization(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::NotFound { .. } => StatusCode::NOT_FOUND,
        }
    }

    pub fn already_exists(subject: &str) -> Self {
        Self::AlreadyExists {
            subject: subject.to_string(),
        }
    }

    pub fn bad_request(message: &str) -> Self {
        Self::BadRequest(message.to_string())
    }

    pub fn invalid_input(message: &str) -> Self {
        Self::InvalidInput(message.to_string())
    }

    pub fn not_found(subject: &str) -> Self {
        Self::NotFound {
            subject: subject.to_string(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.get_status_code();
        let message = self.to_string();

        match self {
            Self::Argon2Hash(error) => error!("Argon2 hash error: {}", error),
            Self::DatabaseConnection(error) => error!("Database connection error: {}", error),
            Self::DatabaseMigrationError(error) => error!("Database migration error: {}", error),
            Self::DatabaseQuery(error) => error!("Database query error: {}", error),
            Self::JWT(error) => error!("JWT error: {}", error),
            Self::Serialization(error) => error!("Serialization error: {}", error),
            _ => (),
        }

        (status, message).into_response()
    }
}
