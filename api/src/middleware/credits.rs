use crate::repositories::{users::UserRepository, RepositoryFactory};
use crate::utils::pricing::{PricingTier, should_reset_credits};
use crate::errors::AppError;

pub async fn check_and_reset_credits(
    company_id: i64,
    repo_factory: &RepositoryFactory,
) -> Result<(), AppError> {
    let user_repo = repo_factory.create_user_repository();
    let company = user_repo.get_company_by_id(company_id)
        .map_err(|e| AppError::Database(e))?;
    
    // Check if credits need to be reset
    if should_reset_credits(company.credits_reset_date) {
        let pricing_tier = PricingTier::from_str(&company.pricing_tier);
        
        // Only reset for non-enterprise tiers
        if !matches!(pricing_tier, PricingTier::Enterprise) {
            user_repo.reset_company_credits(company_id, &company.pricing_tier)
                .map_err(|e| AppError::Database(e))?;
            log::info!("Reset API credits for company {} ({})", company_id, company.pricing_tier);
        }
    }
    
    Ok(())
}

pub async fn check_api_credits(
    company_id: i64,
    repo_factory: &RepositoryFactory,
) -> Result<bool, AppError> {
    // First check if credits need reset
    check_and_reset_credits(company_id, repo_factory).await?;
    
    let user_repo = repo_factory.create_user_repository();
    let company = user_repo.get_company_by_id(company_id)
        .map_err(|e| AppError::Database(e))?;
    
    // Enterprise has unlimited credits
    if company.pricing_tier == "enterprise" {
        return Ok(true);
    }
    
    // Check if company has credits remaining
    Ok(company.api_credits > 0)
}

pub async fn deduct_api_credit(
    company_id: i64,
    repo_factory: &RepositoryFactory,
) -> Result<(), AppError> {
    let user_repo = repo_factory.create_user_repository();
    let company = user_repo.get_company_by_id(company_id)
        .map_err(|e| AppError::Database(e))?;
    
    // Don't deduct for enterprise
    if company.pricing_tier == "enterprise" {
        return Ok(());
    }
    
    user_repo.deduct_api_credit(company_id)
        .map_err(|e| AppError::Database(e))?;
    
    Ok(())
}