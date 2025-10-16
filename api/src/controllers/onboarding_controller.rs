use crate::errors::AppError;
use crate::models::users::{ApiKey, Company, NewApiKey, NewCompany, NewTeamMember};
use crate::repositories::{users::UserRepository, RepositoryFactory};
use crate::utils::utils::service_response;
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CompleteOnboardingRequest {
    pub company_name: String,
    pub website: Option<String>,
    pub industry: String,
    pub sending_domain: String,
    pub from_name: String,
    pub from_email: String,
    pub user_id: i64,
}

#[derive(Serialize)]
pub struct OnboardingResponse {
    pub company_id: i64,
    pub api_key: String,
    pub message: String,
}

pub struct OnboardingController;

impl OnboardingController {
    pub async fn complete_onboarding(
        req: web::Json<CompleteOnboardingRequest>,
        repo_factory: web::Data<RepositoryFactory>,
    ) -> Result<HttpResponse, AppError> {
        if req.company_name.is_empty() || req.industry.is_empty() {
            return Err(AppError::Validation(
                "Company name and industry are required".to_string(),
            ));
        }

        if req.sending_domain.is_empty() || req.from_name.is_empty() || req.from_email.is_empty() {
            return Err(AppError::Validation(
                "All email configuration fields are required".to_string(),
            ));
        }

        let user_repo = repo_factory.create_user_repository();

        // Find industry ID by name (for now, we'll use a default or create logic)
        let industry_id = match req.industry.as_str() {
            "Technology" => Some(1),
            "E-commerce" => Some(2),
            "Healthcare" => Some(3),
            "Finance" => Some(4),
            "Education" => Some(5),
            _ => Some(6), // Other
        };

        // Create company with free tier and initial credits
        use crate::utils::pricing::{PricingTier, get_next_reset_date};
        
        let pricing_tier = PricingTier::Free;
        let initial_credits = pricing_tier.monthly_credits();
        let next_reset = get_next_reset_date();
        
        let new_company = NewCompany {
            company_name: req.company_name.clone(),
            company_address: None,
            website: req.website.clone(),
            sending_domain: Some(req.sending_domain.clone()),
            default_from_name: Some(req.from_name.clone()),
            default_from_email: Some(req.from_email.clone()),
            owner_id: req.user_id,
            industry_id,
            pricing_tier: pricing_tier.to_string(),
            api_credits: initial_credits,
            credits_reset_date: next_reset,
        };

        let company = user_repo.create_company(new_company)?;

        // Generate API key
        let api_key_value = format!("mn_live_{}", Uuid::new_v4().simple());
        let new_api_key = NewApiKey {
            name: "Default API Key".to_string(),
            api_key: api_key_value.clone(),
            company_id: company.id,
            permission: Some("Full Access".to_string()),
            created_at: chrono::Utc::now(),
            is_active: true,
        };

        user_repo.create_api_key(new_api_key)?;

        // Create team member with Owner role
        let new_team_member = NewTeamMember {
            role: "Owner".to_string(),
            company_id: company.id,
            user_id: req.user_id,
        };

        user_repo.create_team_member(new_team_member)?;

        let response = OnboardingResponse {
            company_id: company.id,
            api_key: api_key_value,
            message: "Onboarding completed successfully".to_string(),
        };

        Ok(service_response(
            201,
            "Onboarding completed successfully",
            true,
            Some(serde_json::to_value(response).unwrap()),
        ))
    }
}
