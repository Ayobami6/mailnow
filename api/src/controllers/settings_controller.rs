use actix_web::{web, HttpResponse};
use serde::Deserialize;
use crate::utils::utils::service_response;
use crate::errors::AppError;
use crate::auth::jwt::Claims;
use crate::repositories::{users::UserRepository, RepositoryFactory};
use argon2::password_hash::{rand_core::OsRng, SaltString};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};

#[derive(Deserialize)]
pub struct UpdateCompanyRequest {
    pub company_name: String,
    pub website: Option<String>,
    pub company_address: Option<String>,
    pub default_from_email: Option<String>,
    pub default_from_name: Option<String>,
}

#[derive(Deserialize)]
pub struct ChangePasswordRequest {
    pub current_password: String,
    pub new_password: String,
}

pub struct SettingsController;

impl SettingsController {
    pub async fn update_company_profile(
        claims: web::ReqData<Claims>,
        req: web::Json<UpdateCompanyRequest>,
        repo_factory: web::Data<RepositoryFactory>,
    ) -> Result<HttpResponse, AppError> {
        let user_repo = repo_factory.create_user_repository();
        let user_id = claims.into_inner().user_id;

        // Get user's company through team membership
        let team_members = user_repo.get_team_members_by_user(user_id)?;
        let company_id = team_members.first()
            .ok_or_else(|| AppError::Validation("User not associated with any company".to_string()))?
            .company_id;

        // Get current company data
        let mut company = user_repo.get_company_by_id(company_id)?;
        
        // Update company fields
        company.company_name = req.company_name.clone();
        if let Some(website) = &req.website {
            company.website = Some(website.clone());
        }
        if let Some(address) = &req.company_address {
            company.company_address = Some(address.clone());
        }
        if let Some(from_email) = &req.default_from_email {
            company.default_from_email = Some(from_email.clone());
        }
        if let Some(from_name) = &req.default_from_name {
            company.default_from_name = Some(from_name.clone());
        }

        user_repo.update_company(company_id, &company)?;

        Ok(service_response(
            200,
            "Company profile updated successfully",
            true,
            None,
        ))
    }

    pub async fn change_password(
        claims: web::ReqData<Claims>,
        req: web::Json<ChangePasswordRequest>,
        repo_factory: web::Data<RepositoryFactory>,
    ) -> Result<HttpResponse, AppError> {
        let user_repo = repo_factory.create_user_repository();
        let user_id = claims.into_inner().user_id;

        // Get current user
        let mut user = user_repo.get_user_by_id(user_id)?;
        
        // Verify current password
        let argon2 = Argon2::default();
        let parsed_hash = PasswordHash::new(&user.password)
            .map_err(|_| AppError::Validation("Invalid password hash format".to_string()))?;
        
        argon2
            .verify_password(req.current_password.as_bytes(), &parsed_hash)
            .map_err(|_| AppError::Validation("Current password is incorrect".to_string()))?;

        // Hash new password
        let salt = SaltString::generate(&mut OsRng);
        let hashed_password = argon2
            .hash_password(req.new_password.as_bytes(), &salt)
            .map_err(|e| AppError::Validation(format!("Failed to hash password: {}", e)))?
            .to_string();

        // Update password
        user.password = hashed_password;
        user_repo.update_user(user_id, &user)?;

        Ok(service_response(
            200,
            "Password updated successfully",
            true,
            None,
        ))
    }
}