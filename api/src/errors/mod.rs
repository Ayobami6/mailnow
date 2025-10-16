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
                HttpResponse::InternalServerError().json(serde_json::json!({
                    "status_code": 500,
                    "message": "Database error occurred",
                    "success": false,
                    "data": null
                }))
            }
            AppError::PasswordHash(e) => {
                log::error!("Password hash error: {}", e);
                HttpResponse::InternalServerError().json(serde_json::json!({
                    "status_code": 500,
                    "message": "Password processing failed",
                    "success": false,
                    "data": null
                }))
            }
            AppError::Jwt(e) => {
                log::error!("JWT error: {:?}", e);
                HttpResponse::Unauthorized().json(serde_json::json!({
                    "status_code": 401,
                    "message": "Invalid token",
                    "success": false,
                    "data": null
                }))
            }
            AppError::Validation(msg) => {
                log::warn!("Validation error: {}", msg);
                HttpResponse::BadRequest().json(serde_json::json!({
                    "status_code": 400,
                    "message": msg,
                    "success": false,
                    "data": null
                }))
            }
            AppError::Unauthorized => {
                log::warn!("Unauthorized access attempt");
                HttpResponse::Unauthorized().json(serde_json::json!({
                    "status_code": 401,
                    "message": "Authentication failed",
                    "success": false,
                    "data": null
                }))
            }
            AppError::UserNotFound => {
                log::warn!("User not found");
                HttpResponse::NotFound().json(serde_json::json!({
                    "status_code": 404,
                    "message": "User not found",
                    "success": false,
                    "data": null
                }))
            }
            AppError::UserExists => {
                log::warn!("User already exists");
                HttpResponse::Conflict().json(serde_json::json!({
                    "status_code": 409,
                    "message": "User already exists",
                    "success": false,
                    "data": null
                }))
            }
            AppError::Internal => {
                log::error!("Internal server error");
                HttpResponse::InternalServerError().json(serde_json::json!({
                    "status_code": 500,
                    "message": "Internal server error",
                    "success": false,
                    "data": null
                }))
            }
        }
    }
}