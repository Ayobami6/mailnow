use crate::auth::jwt::Claims;
use crate::errors::AppError;
use crate::models::users::NewSmtpProfile;
use crate::repositories::{users::UserRepository, RepositoryFactory};
use crate::utils::utils::service_response;
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct SmtpProfileResponse {
    pub id: i64,
    pub smtp_username: String,
    pub smtp_server: String,
    pub smtp_port: i32,
    pub is_default: bool,
    pub created_at: String,
}

#[derive(Deserialize)]
pub struct CreateSmtpProfileRequest {
    pub smtp_username: String,
    pub smtp_password: String,
    pub smtp_server: String,
    pub smtp_port: i32,
    pub is_default: Option<bool>,
}

#[derive(Deserialize)]
pub struct UpdateSmtpProfileRequest {
    pub smtp_username: String,
    pub smtp_password: String,
    pub smtp_server: String,
    pub smtp_port: i32,
    pub is_default: Option<bool>,
}

pub struct SmtpController;

impl SmtpController {
    pub async fn get_smtp_profiles(
        claims: web::ReqData<Claims>,
        repo_factory: web::Data<RepositoryFactory>,
    ) -> Result<HttpResponse, AppError> {
        let user_repo = repo_factory.create_user_repository();
        let user_id = claims.into_inner().user_id;

        // Get user's company through team membership
        let team_members = user_repo.get_team_members_by_user(user_id)?;
        let company_id = team_members.first()
            .ok_or_else(|| AppError::Validation("User not associated with any company".to_string()))?
            .company_id;

        let profiles = user_repo.get_smtp_profiles_by_company(company_id)?;
        let response_profiles: Vec<SmtpProfileResponse> = profiles.into_iter().map(|profile| {
            SmtpProfileResponse {
                id: profile.id,
                smtp_username: profile.smtp_username,
                smtp_server: profile.smtp_server,
                smtp_port: profile.smtp_port,
                is_default: profile.is_default,
                created_at: profile.created_at.format("%Y-%m-%d %H:%M").to_string(),
            }
        }).collect();

        Ok(service_response(
            200,
            "SMTP profiles retrieved successfully",
            true,
            Some(serde_json::to_value(response_profiles).unwrap()),
        ))
    }

    pub async fn create_smtp_profile(
        claims: web::ReqData<Claims>,
        req: web::Json<CreateSmtpProfileRequest>,
        repo_factory: web::Data<RepositoryFactory>,
    ) -> Result<HttpResponse, AppError> {
        let user_repo = repo_factory.create_user_repository();
        let user_id = claims.into_inner().user_id;

        // Get user's company through team membership
        let team_members = user_repo.get_team_members_by_user(user_id)?;
        let company_id = team_members.first()
            .ok_or_else(|| AppError::Validation("User not associated with any company".to_string()))?
            .company_id;

        let new_profile = NewSmtpProfile {
            company_id,
            smtp_username: req.smtp_username.clone(),
            smtp_password: req.smtp_password.clone(),
            smtp_server: req.smtp_server.clone(),
            smtp_port: req.smtp_port,
            is_default: req.is_default.unwrap_or(false),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        let created_profile = user_repo.create_smtp_profile(new_profile)?;

        let response = SmtpProfileResponse {
            id: created_profile.id,
            smtp_username: created_profile.smtp_username,
            smtp_server: created_profile.smtp_server,
            smtp_port: created_profile.smtp_port,
            is_default: created_profile.is_default,
            created_at: created_profile.created_at.format("%Y-%m-%d %H:%M").to_string(),
        };

        Ok(service_response(
            201,
            "SMTP profile created successfully",
            true,
            Some(serde_json::to_value(response).unwrap()),
        ))
    }

    pub async fn update_smtp_profile(
        claims: web::ReqData<Claims>,
        path: web::Path<i64>,
        req: web::Json<UpdateSmtpProfileRequest>,
        repo_factory: web::Data<RepositoryFactory>,
    ) -> Result<HttpResponse, AppError> {
        let user_repo = repo_factory.create_user_repository();
        let user_id = claims.into_inner().user_id;
        let profile_id = path.into_inner();

        // Get user's company through team membership
        let team_members = user_repo.get_team_members_by_user(user_id)?;
        let company_id = team_members.first()
            .ok_or_else(|| AppError::Validation("User not associated with any company".to_string()))?
            .company_id;

        // Get existing profile to ensure it belongs to user's company
        let mut existing_profile = user_repo.get_smtp_profile_by_id(profile_id)?;
        if existing_profile.company_id != company_id {
            return Err(AppError::Validation("SMTP profile not found".to_string()));
        }

        // Update profile fields
        existing_profile.smtp_username = req.smtp_username.clone();
        existing_profile.smtp_password = req.smtp_password.clone();
        existing_profile.smtp_server = req.smtp_server.clone();
        existing_profile.smtp_port = req.smtp_port;
        existing_profile.is_default = req.is_default.unwrap_or(existing_profile.is_default);

        let updated_profile = user_repo.update_smtp_profile(profile_id, &existing_profile)?;

        let response = SmtpProfileResponse {
            id: updated_profile.id,
            smtp_username: updated_profile.smtp_username,
            smtp_server: updated_profile.smtp_server,
            smtp_port: updated_profile.smtp_port,
            is_default: updated_profile.is_default,
            created_at: updated_profile.created_at.format("%Y-%m-%d %H:%M").to_string(),
        };

        Ok(service_response(
            200,
            "SMTP profile updated successfully",
            true,
            Some(serde_json::to_value(response).unwrap()),
        ))
    }

    pub async fn delete_smtp_profile(
        claims: web::ReqData<Claims>,
        path: web::Path<i64>,
        repo_factory: web::Data<RepositoryFactory>,
    ) -> Result<HttpResponse, AppError> {
        let user_repo = repo_factory.create_user_repository();
        let user_id = claims.into_inner().user_id;
        let profile_id = path.into_inner();

        // Get user's company through team membership
        let team_members = user_repo.get_team_members_by_user(user_id)?;
        let company_id = team_members.first()
            .ok_or_else(|| AppError::Validation("User not associated with any company".to_string()))?
            .company_id;

        let deleted_count = user_repo.delete_smtp_profile(profile_id, company_id)?;
        if deleted_count == 0 {
            return Err(AppError::Validation("SMTP profile not found".to_string()));
        }

        Ok(service_response(
            200,
            "SMTP profile deleted successfully",
            true,
            None,
        ))
    }

    pub async fn set_default_smtp_profile(
        claims: web::ReqData<Claims>,
        path: web::Path<i64>,
        repo_factory: web::Data<RepositoryFactory>,
    ) -> Result<HttpResponse, AppError> {
        let user_repo = repo_factory.create_user_repository();
        let user_id = claims.into_inner().user_id;
        let profile_id = path.into_inner();

        // Get user's company through team membership
        let team_members = user_repo.get_team_members_by_user(user_id)?;
        let company_id = team_members.first()
            .ok_or_else(|| AppError::Validation("User not associated with any company".to_string()))?
            .company_id;

        let updated_profile = user_repo.set_default_smtp_profile(profile_id, company_id)?;

        let response = SmtpProfileResponse {
            id: updated_profile.id,
            smtp_username: updated_profile.smtp_username,
            smtp_server: updated_profile.smtp_server,
            smtp_port: updated_profile.smtp_port,
            is_default: updated_profile.is_default,
            created_at: updated_profile.created_at.format("%Y-%m-%d %H:%M").to_string(),
        };

        Ok(service_response(
            200,
            "Default SMTP profile set successfully",
            true,
            Some(serde_json::to_value(response).unwrap()),
        ))
    }
}