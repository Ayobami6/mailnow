use actix_web::{web, HttpResponse};
use argon2::password_hash::{rand_core::OsRng, SaltString};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use tokio;
use crate::auth::jwt::JwtService;
use crate::errors::AppError;
use crate::models::users::NewUser;
use crate::repositories::{users::UserRepository, RepositoryFactory};
use crate::services::email_service::EmailService;
use crate::utils::utils::{get_env, service_response};
use crate::utils::template::load_template;
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
    ) -> Result<HttpResponse, AppError> {
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

        Ok(service_response(
            201,
            "User created successfully",
            true,
            Some(serde_json::to_value(user_response).unwrap()),
        ))
    }

    pub async fn login(
        req: web::Json<LoginRequest>,
        repo_factory: web::Data<RepositoryFactory>,
        jwt_service: web::Data<JwtService>,
    ) -> Result<HttpResponse, AppError> {
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

        Ok(service_response(
            200,
            "Login successful",
            true,
            Some(serde_json::to_value(auth_response).unwrap()),
        ))
    }

    pub async fn verify_email_send(
        req: web::Json<VerifyEmailRequest>,
    ) -> Result<HttpResponse, AppError> {
        if req.email.is_empty() {
            return Err(AppError::Validation("Email is required".to_string()));
        }

        let token = Uuid::new_v4().to_string();
        let verification_link = format!(
            "http://localhost:3000/verify-email?token={}&success=true",
            token
        );

        let email = req.email.clone();
        let link = verification_link.clone();

        // Queue background email task
        tokio::spawn(async move {
            let email_service = EmailService::new();
            let smtp_server = get_env("SMTP_HOST", "smtp.gmail.com");
            let smtp_username = get_env("SMTP_USERNAME", "");
            let smtp_password = get_env("SMTP_PASSWORD", "");
            let mut template_vars = HashMap::new();
            template_vars.insert("verification_link", link.as_str());
            
            let html_content = match load_template("verify_email", template_vars) {
                Ok(content) => content,
                Err(e) => {
                    log::error!("Failed to load email template: {:?}", e);
                    format!("Please verify your email by clicking this link: {}", &link)
                }
            };

            let result = email_service
                .send_email(
                    &smtp_server,
                    &smtp_username,
                    &smtp_password,
                    "MailNow <noreply@mailnow.dev>",
                    &email,
                    "Verify Your Email Address - MailNow",
                    &html_content,
                    true,
                )
                .await;

            if let Err(e) = result {
                log::error!("Failed to send verification email to {}: {:?}", email, e);
            } else {
                log::info!("Verification email sent successfully to {}", email);
            }
        });

        let response = VerifyEmailResponse {
            message: "Verification email queued successfully".to_string(),
            verification_link,
        };

        Ok(service_response(
            200,
            "Verification email queued",
            true,
            Some(serde_json::to_value(response).unwrap()),
        ))
    }

    pub async fn verify_email_token(
        query: web::Query<HashMap<String, String>>,
    ) -> Result<HttpResponse, AppError> {
        let _token = query
            .get("token")
            .ok_or_else(|| AppError::Validation("Missing token".to_string()))?;

        Ok(service_response(
            200,
            "Email verified successfully",
            true,
            None,
        ))
    }
}
