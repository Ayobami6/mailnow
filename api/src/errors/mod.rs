use crate::utils::utils::service_response;
use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] diesel::result::Error),

    #[error("Password hashing error: {0}")]
    PasswordHash(String),

    #[error("JWT error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Authentication failed")]
    Unauthorized,

    #[error("User not found")]
    UserNotFound,

    #[error("User already exists")]
    UserExists,

    #[error("Internal server error")]
    Internal,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        log::error!("API Error: {}", self);

        match self {
            AppError::Database(e) => {
                log::error!("Database error: {:?}", e);
                service_response(500, "Database error occurred", false, None)
            }
            AppError::PasswordHash(e) => {
                log::error!("Password hash error: {}", e);
                service_response(500, "Password processing failed", false, None)
            }
            AppError::Jwt(e) => {
                log::error!("JWT error: {:?}", e);
                service_response(401, "Invalid token", false, None)
            }
            AppError::Validation(msg) => {
                log::warn!("Validation error: {}", msg);
                service_response(400, msg, false, None)
            }
            AppError::Unauthorized => {
                log::warn!("Unauthorized access attempt");
                service_response(401, "Authentication failed", false, None)
            }
            AppError::UserNotFound => {
                log::warn!("User not found");
                service_response(404, "User not found", false, None)
            }
            AppError::UserExists => {
                log::warn!("User already exists");
                service_response(409, "User already exists", false, None)
            }
            AppError::Internal => {
                log::error!("Internal server error");
                service_response(500, "Internal server error", false, None)
            }
        }
    }
}
