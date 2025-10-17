use crate::auth::jwt::Claims;
use crate::errors::AppError;
use crate::repositories::{users::UserRepository, RepositoryFactory};
use crate::utils::utils::service_response;
use actix_web::{web, HttpResponse};
use serde::Serialize;

#[derive(Serialize)]
pub struct UserProfileResponse {
    pub user: UserInfo,
    pub company: Option<CompanyInfo>,
}

#[derive(Serialize)]
pub struct UserInfo {
    pub id: i64,
    pub email: String,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub email_verified: bool,
}

#[derive(Serialize)]
pub struct CompanyInfo {
    pub id: i64,
    pub company_name: String,
    pub website: Option<String>,
    pub pricing_tier: String,
    pub api_credits: i64,
    pub sending_domain: Option<String>,
    pub member_role: Option<String>,
}

pub struct UserController;

impl UserController {
    pub async fn get_profile(
        claims: web::ReqData<Claims>,
        repo_factory: web::Data<RepositoryFactory>,
    ) -> Result<HttpResponse, AppError> {
        let user_repo = repo_factory.create_user_repository();

        // Get user info
        let user = user_repo.get_user_by_id(claims.into_inner().user_id)?;

        // Get user's company through team membership
        let company_info = if let Ok(team_members) = user_repo.get_team_members_by_user(user.id) {
            if let Some(team_member) = team_members.first() {
                if let Ok(company) = user_repo.get_company_by_id(team_member.company_id) {
                    Some(CompanyInfo {
                        id: company.id,
                        company_name: company.company_name,
                        website: company.website,
                        pricing_tier: company.pricing_tier,
                        api_credits: company.api_credits,
                        sending_domain: company.sending_domain,
                        member_role: Some(team_member.role.clone()),
                    })
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };

        let profile = UserProfileResponse {
            user: UserInfo {
                id: user.id,
                email: user.email,
                firstname: user.firstname,
                lastname: user.lastname,
                email_verified: user.email_verified,
            },
            company: company_info,
        };

        Ok(service_response(
            200,
            "Profile retrieved successfully",
            true,
            Some(serde_json::to_value(profile).unwrap()),
        ))
    }
}
