use crate::errors::AppError;
use crate::models::users::{NewTeamMember, NewUser, TeamMember};
use crate::repositories::users::UserRepository;
use crate::services::email_service::EmailService;
use crate::utils::redis_verification::{generate_verification_token, store_verification_token};
use crate::utils::template::load_template;
use crate::utils::utils::get_env;
use crate::utils::utils::service_response;
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio;

#[derive(Deserialize)]
pub struct InviteTeamMemberRequest {
    pub email: String,
    pub role: String,
    pub company_id: i64,
}

#[derive(Deserialize)]
pub struct AcceptInviteRequest {
    pub token: String,
    pub firstname: String,
    pub lastname: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct TeamMemberResponse {
    pub id: i64,
    pub email: String,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub role: String,
    pub created_at: String,
}

#[derive(Serialize)]
pub struct InviteResponse {
    pub message: String,
    pub invite_link: String,
}

pub struct TeamController;

impl TeamController {
    pub async fn invite_member(
        req: web::Json<InviteTeamMemberRequest>,
        repo_factory: web::Data<crate::repositories::RepositoryFactory>,
    ) -> Result<HttpResponse, AppError> {
        if req.email.is_empty() || req.role.is_empty() {
            return Err(AppError::Validation(
                "Email and role are required".to_string(),
            ));
        }

        let user_repo = repo_factory.create_user_repository();

        // Check if user already exists
        if let Ok(_) = user_repo.get_user_by_email(&req.email) {
            return Err(AppError::Validation("User already exists".to_string()));
        }

        // Generate invitation token
        let token = generate_verification_token();
        let invite_link = format!("http://localhost:3000/team/accept-invite?token={}", token);

        // Store invitation data in Redis (company_id:role:email)
        let invite_data = format!("{}:{}:{}", req.company_id, req.role, req.email);
        if let Err(e) = store_verification_token(0, &format!("invite:{}", token)).await {
            log::error!("Failed to store invitation token: {:?}", e);
            return Err(AppError::Internal);
        }

        // Store invite data separately
        use crate::config::redis::get_redis_connection;
        use redis::Commands;

        let mut conn = get_redis_connection()
            .await
            .map_err(|_| AppError::Internal)?;
        let _: () = conn
            .set_ex(format!("invite_data:{}", token), invite_data, 86400)
            .map_err(|_| AppError::Internal)?;

        let email = req.email.clone();
        let link = invite_link.clone();

        // Send invitation email
        tokio::spawn(async move {
            let email_service = EmailService::new();
            let smtp_server = get_env("SMTP_HOST", "smtp.gmail.com");
            let smtp_username = get_env("SMTP_USERNAME", "");
            let smtp_password = get_env("SMTP_PASSWORD", "");
            let mut template_vars = HashMap::new();
            template_vars.insert("invite_link", link.as_str());
            template_vars.insert("role", &req.role);

            let html_content = match load_template("team_invite", template_vars) {
                Ok(content) => content,
                Err(e) => {
                    log::error!("Failed to load email template: {:?}", e);
                    format!(
                        "You've been invited to join as {}. Click here to accept: {}",
                        req.role, &link
                    )
                }
            };

            let result = email_service
                .send_email(
                    &smtp_server,
                    &smtp_username,
                    &smtp_password,
                    "MailNow <noreply@mailnow.dev>",
                    &email,
                    "Team Invitation - MailNow",
                    &html_content,
                    false,
                )
                .await;

            if let Err(e) = result {
                log::error!("Failed to send invitation email to {}: {:?}", email, e);
            } else {
                log::info!("Invitation email sent successfully to {}", email);
            }
        });

        let response = InviteResponse {
            message: "Invitation sent successfully".to_string(),
            invite_link,
        };

        Ok(service_response(
            200,
            "Invitation sent successfully",
            true,
            Some(serde_json::to_value(response).unwrap()),
        ))
    }

    pub async fn accept_invite(
        req: web::Json<AcceptInviteRequest>,
        repo_factory: web::Data<crate::repositories::RepositoryFactory>,
    ) -> Result<HttpResponse, AppError> {
        use crate::config::redis::get_redis_connection;
        use argon2::password_hash::{rand_core::OsRng, SaltString};
        use argon2::{Argon2, PasswordHasher};
        use redis::Commands;

        if req.firstname.is_empty() || req.lastname.is_empty() || req.password.is_empty() {
            return Err(AppError::Validation("All fields are required".to_string()));
        }

        // Get invitation data from Redis
        let mut conn = get_redis_connection()
            .await
            .map_err(|_| AppError::Internal)?;

        let invite_data: String = conn
            .get(format!("invite_data:{}", req.token))
            .map_err(|_| AppError::Validation("Invalid or expired invitation".to_string()))?;

        println!("Invite data: {}", invite_data);

        let parts: Vec<&str> = invite_data.split(':').collect();
        if parts.len() != 3 {
            return Err(AppError::Validation("Invalid invitation data".to_string()));
        }

        let company_id: i64 = parts[0]
            .parse()
            .map_err(|_| AppError::Validation("Invalid company ID".to_string()))?;
        let role = parts[1].to_string();
        let email = parts[2].to_string();

        let user_repo = repo_factory.create_user_repository();

        // Hash password
        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = argon2
            .hash_password(req.password.as_bytes(), &salt)
            .map_err(|e| AppError::PasswordHash(e.to_string()))?
            .to_string();

        // Create user
        let new_user = NewUser {
            email: email.clone(),
            password: password_hash,
            firstname: Some(req.firstname.clone()),
            lastname: Some(req.lastname.clone()),
            user_type: "customer".to_string(),
            is_active: true,
            is_staff: false,
            is_superuser: false,
            mfa_enabled: false,
            email_verified: true, // Auto-verify invited users
            date_joined: chrono::Utc::now(),
        };

        let user = user_repo.create_user(new_user)?;

        // Create team member
        let new_team_member = NewTeamMember {
            role,
            company_id,
            user_id: user.id,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        user_repo.create_team_member(new_team_member)?;

        // Clean up invitation tokens
        let _: () = conn
            .del(format!("invite_data:{}", req.token))
            .map_err(|_| AppError::Internal)?;
        let _: () = conn
            .del(format!("invite:{}", req.token))
            .map_err(|_| AppError::Internal)?;

        Ok(service_response(
            201,
            "Invitation accepted successfully",
            true,
            None,
        ))
    }

    pub async fn list_team_members(
        company_id: web::Path<i64>,
        repo_factory: web::Data<crate::repositories::RepositoryFactory>,
    ) -> Result<HttpResponse, AppError> {
        let user_repo = repo_factory.create_user_repository();
        let team_members = user_repo
            .get_team_members_by_company(company_id.into_inner())
            .map_err(|e| AppError::Database(e))?;

        let mut response_members = Vec::new();
        for member in team_members {
            let user = user_repo
                .get_user_by_id(member.user_id)
                .map_err(|e| AppError::Database(e))?;

            response_members.push(TeamMemberResponse {
                id: member.id,
                email: user.email,
                firstname: user.firstname,
                lastname: user.lastname,
                role: member.role,
                created_at: member.created_at.format("%Y-%m-%d").to_string(),
            });
        }

        Ok(service_response(
            200,
            "Team members retrieved successfully",
            true,
            Some(serde_json::to_value(response_members).unwrap()),
        ))
    }

    pub async fn get_team_member(
        path: web::Path<(i64, i64)>,
        repo_factory: web::Data<crate::repositories::RepositoryFactory>,
    ) -> Result<HttpResponse, AppError> {
        let (company_id, member_id) = path.into_inner();
        let user_repo = repo_factory.create_user_repository();

        let team_members = user_repo
            .get_team_members_by_company(company_id)
            .map_err(|e| AppError::Database(e))?;

        let member = team_members
            .into_iter()
            .find(|m| m.id == member_id)
            .ok_or_else(|| AppError::Validation("Team member not found".to_string()))?;

        let user = user_repo
            .get_user_by_id(member.user_id)
            .map_err(|e| AppError::Database(e))?;

        let response = TeamMemberResponse {
            id: member.id,
            email: user.email,
            firstname: user.firstname,
            lastname: user.lastname,
            role: member.role,
            created_at: member.created_at.format("%Y-%m-%d").to_string(),
        };

        Ok(service_response(
            200,
            "Team member retrieved successfully",
            true,
            Some(serde_json::to_value(response).unwrap()),
        ))
    }
}
