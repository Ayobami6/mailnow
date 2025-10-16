use actix_web::{web, Result};
use argon2::password_hash::{rand_core::OsRng, SaltString};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::auth::jwt::JwtService;
use crate::errors::AppError;
use crate::models::users::{NewUser, User};
use crate::repositories::{users::UserRepository, RepositoryFactory};
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    pub firstname: String,
    pub lastname: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub id: i64,
    pub email: String,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub email_verified: bool,
}

#[derive(Deserialize)]
pub struct VerifyEmailRequest {
    pub email: String,
}

#[derive(Serialize)]
pub struct VerifyEmailResponse {
    pub message: String,
    pub verification_link: String,
}

pub async fn signup(
    req: web::Json<SignupRequest>,
    repo_factory: web::Data<RepositoryFactory>,
) -> Result<web::Json<serde_json::Value>, AppError> {
    log::info!("Signup attempt for email: {}", req.email);
    
    // Validate input
    if req.email.is_empty() || req.password.is_empty() {
        log::warn!("Signup validation failed: empty email or password");
        return Err(AppError::Validation("Email and password are required".to_string()));
    }
    
    if req.password.len() < 8 {
        log::warn!("Signup validation failed: password too short");
        return Err(AppError::Validation("Password must be at least 8 characters".to_string()));
    }

    let user_repo = repo_factory.create_user_repository();

    // Check if user exists
    match user_repo.get_user_by_email(&req.email) {
        Ok(_) => {
            log::warn!("Signup failed: user already exists for email: {}", req.email);
            return Err(AppError::UserExists);
        }
        Err(diesel::result::Error::NotFound) => {
            log::debug!("Email available for signup: {}", req.email);
        }
        Err(e) => {
            log::error!("Database error during user lookup: {:?}", e);
            return Err(AppError::Database(e));
        }
    }

    // Hash password
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = argon2
        .hash_password(req.password.as_bytes(), &salt)
        .map_err(|e| {
            log::error!("Password hashing failed: {:?}", e);
            AppError::PasswordHash(e.to_string())
        })?
        .to_string();

    let new_user = NewUser {
        email: req.email.clone(),
        password: password_hash,
        firstname: Some(req.firstname.clone()),
        lastname: Some(req.lastname.clone()),
        user_type: "customer".to_string(),
    };

    let user = user_repo.create_user(new_user)?;
    log::info!("User created successfully with ID: {}", user.id);

    let user_response = UserResponse {
        id: user.id,
        email: user.email,
        firstname: user.firstname,
        lastname: user.lastname,
        email_verified: user.email_verified,
    };

    Ok(web::Json(serde_json::json!({
        "status_code": 201,
        "message": "User created successfully",
        "success": true,
        "data": user_response
    })))
}

pub async fn login(
    req: web::Json<LoginRequest>,
    repo_factory: web::Data<RepositoryFactory>,
    jwt_service: web::Data<JwtService>,
) -> Result<web::Json<serde_json::Value>, AppError> {
    log::info!("Login attempt for email: {}", req.email);
    
    // Validate input
    if req.email.is_empty() || req.password.is_empty() {
        log::warn!("Login validation failed: empty email or password");
        return Err(AppError::Validation("Email and password are required".to_string()));
    }

    let user_repo = repo_factory.create_user_repository();

    let user = user_repo.get_user_by_email(&req.email).map_err(|e| {
        log::warn!("Login failed: user not found for email: {}", req.email);
        match e {
            diesel::result::Error::NotFound => AppError::Unauthorized,
            _ => AppError::Database(e),
        }
    })?;

    // Verify password
    let argon2 = Argon2::default();
    let parsed_hash = PasswordHash::new(&user.password).map_err(|e| {
        log::error!("Invalid password hash for user {}: {:?}", user.id, e);
        AppError::Internal
    })?;

    argon2
        .verify_password(req.password.as_bytes(), &parsed_hash)
        .map_err(|_| {
            log::warn!("Login failed: invalid password for user: {}", user.id);
            AppError::Unauthorized
        })?;

    // Generate JWT token
    let token = jwt_service.generate_token(user.id, &user.email)?;
    log::info!("Login successful for user: {}", user.id);

    let auth_response = AuthResponse {
        token,
        user: UserResponse {
            id: user.id,
            email: user.email,
            firstname: user.firstname,
            lastname: user.lastname,
            email_verified: user.email_verified,
        },
    };

    Ok(web::Json(serde_json::json!({
        "status_code": 200,
        "message": "Login successful",
        "success": true,
        "data": auth_response
    })))
}

pub async fn verify_email_send(
    req: web::Json<VerifyEmailRequest>,
) -> Result<web::Json<serde_json::Value>, AppError> {
    log::info!("Verification email request for: {}", req.email);
    
    if req.email.is_empty() {
        return Err(AppError::Validation("Email is required".to_string()));
    }

    // Generate verification token
    let token = Uuid::new_v4().to_string();
    let verification_link = format!(
        "http://localhost:3000/verify-email?token={}&success=true",
        token
    );
    
    log::debug!("Generated verification token for {}: {}", req.email, token);

    let response = VerifyEmailResponse {
        message: "Verification email sent successfully".to_string(),
        verification_link,
    };

    Ok(web::Json(serde_json::json!({
        "status_code": 200,
        "message": "Verification email sent",
        "success": true,
        "data": response
    })))
}

pub async fn verify_email_token(
    query: web::Query<HashMap<String, String>>,
) -> Result<web::Json<serde_json::Value>, AppError> {
    let token = query.get("token").ok_or_else(|| {
        log::warn!("Email verification failed: missing token");
        AppError::Validation("Missing token".to_string())
    })?;

    log::info!("Email verification attempt with token: {}", token);

    // In a real app, you'd validate the token against database
    // For demo, we'll just return success
    log::info!("Email verification successful for token: {}", token);

    Ok(web::Json(serde_json::json!({
        "status_code": 200,
        "message": "Email verified successfully",
        "success": true,
        "data": null
    })))
}