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

pub struct AuthController;

impl AuthController {
    pub async fn signup(
        req: web::Json<SignupRequest>,
        repo_factory: web::Data<RepositoryFactory>,
    ) -> Result<web::Json<serde_json::Value>, AppError> {
        if req.email.is_empty() || req.password.is_empty() {
            return Err(AppError::Validation(
                "Email and password are required".to_string(),
            ));
        }

        if req.password.len() < 8 {
            return Err(AppError::Validation(
                "Password must be at least 8 characters".to_string(),
            ));
        }

        let user_repo = repo_factory.create_user_repository();

        match user_repo.get_user_by_email(&req.email) {
            Ok(_) => return Err(AppError::UserExists),
            Err(diesel::result::Error::NotFound) => {}
            Err(e) => return Err(AppError::Database(e)),
        }

        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = argon2
            .hash_password(req.password.as_bytes(), &salt)
            .map_err(|e| AppError::PasswordHash(e.to_string()))?
            .to_string();

        let new_user = NewUser {
            email: req.email.clone(),
            password: password_hash,
            firstname: Some(req.firstname.clone()),
            lastname: Some(req.lastname.clone()),
            user_type: "customer".to_string(),
            is_active: true,
            is_staff: false,
            is_superuser: false,
            mfa_enabled: false,
            email_verified: false,
            date_joined: chrono::Utc::now(),
        };

        let user = user_repo.create_user(new_user)?;

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
        if req.email.is_empty() || req.password.is_empty() {
            return Err(AppError::Validation(
                "Email and password are required".to_string(),
            ));
        }

        let user_repo = repo_factory.create_user_repository();
        let user = user_repo
            .get_user_by_email(&req.email)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => AppError::Unauthorized,
                _ => AppError::Database(e),
            })?;

        let argon2 = Argon2::default();
        let parsed_hash = PasswordHash::new(&user.password).map_err(|_| AppError::Internal)?;

        argon2
            .verify_password(req.password.as_bytes(), &parsed_hash)
            .map_err(|_| AppError::Unauthorized)?;

        let token = jwt_service.generate_token(user.id, &user.email)?;

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
        if req.email.is_empty() {
            return Err(AppError::Validation("Email is required".to_string()));
        }

        let token = Uuid::new_v4().to_string();
        let verification_link = format!(
            "http://localhost:3000/verify-email?token={}&success=true",
            token
        );

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
        let _token = query
            .get("token")
            .ok_or_else(|| AppError::Validation("Missing token".to_string()))?;

        Ok(web::Json(serde_json::json!({
            "status_code": 200,
            "message": "Email verified successfully",
            "success": true,
            "data": null
        })))
    }
}
