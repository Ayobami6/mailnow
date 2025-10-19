use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use crate::utils::utils::service_response;
use crate::errors::AppError;
use crate::repositories::users::UserRepository;
use crate::auth::jwt::Claims;

#[derive(Serialize)]
pub struct DashboardStats {
    pub emails_sent: i64,
    pub delivery_rate: f64,
    pub api_calls: i64,
    pub active_users: i64,
    pub api_credits_remaining: i64,
    pub pricing_tier: String,
    pub credits_reset_date: String,
}

#[derive(Serialize)]
pub struct RecentActivity {
    pub status: String,
    pub email: String,
    pub time: String,
}

#[derive(Serialize)]
pub struct SystemStatus {
    pub api_status: String,
    pub email_delivery: String,
    pub webhook_processing: String,
}

pub struct DashboardController;

impl DashboardController {
    pub async fn get_stats(
        claims: web::ReqData<Claims>,
        repo_factory: web::Data<crate::repositories::RepositoryFactory>,
    ) -> Result<HttpResponse, AppError> {
        let user_repo = repo_factory.create_user_repository();
        let user_id = claims.into_inner().user_id;

        // Get user's company through team membership
        let team_members = user_repo.get_team_members_by_user(user_id)?;
        let company_id = team_members.first()
            .ok_or_else(|| AppError::Validation("User not associated with any company".to_string()))?
            .company_id;
        
        let company = user_repo.get_company_by_id(company_id)
            .map_err(|e| AppError::Database(e))?;
        
        // Get email statistics from email logs
        let (total_emails, sent_emails, _queued, _failed) = user_repo.get_email_log_stats(company_id)?;
        let delivery_rate = if total_emails > 0 {
            ((sent_emails as f64 / total_emails as f64) * 100.0 * 100.0).round() / 100.0
        } else {
            0.0
        };
        
        let stats = DashboardStats {
            emails_sent: sent_emails,
            delivery_rate,
            api_calls: total_emails, // Total API calls = total emails attempted
            active_users: 1, // Current user count (could be enhanced to count team members)
            api_credits_remaining: company.api_credits,
            pricing_tier: company.pricing_tier,
            credits_reset_date: company.credits_reset_date.format("%Y-%m-%d").to_string(),
        };

        Ok(service_response(
            200,
            "Dashboard stats retrieved successfully",
            true,
            Some(serde_json::to_value(stats).unwrap()),
        ))
    }

    pub async fn get_recent_activity(
        claims: web::ReqData<Claims>,
        repo_factory: web::Data<crate::repositories::RepositoryFactory>,
    ) -> Result<HttpResponse, AppError> {
        let user_repo = repo_factory.create_user_repository();
        let user_id = claims.into_inner().user_id;

        // Get user's company through team membership
        let team_members = user_repo.get_team_members_by_user(user_id)?;
        let company_id = team_members.first()
            .ok_or_else(|| AppError::Validation("User not associated with any company".to_string()))?
            .company_id;

        // Get recent email logs (limit to 5 most recent)
        let email_logs = user_repo.get_email_logs_by_company(company_id)?;
        let activities: Vec<RecentActivity> = email_logs.into_iter().take(5).map(|log| {
            let time_diff = chrono::Utc::now().signed_duration_since(log.created_at);
            let time_str = if time_diff.num_minutes() < 60 {
                format!("{} minutes ago", time_diff.num_minutes().max(1))
            } else if time_diff.num_hours() < 24 {
                format!("{} hours ago", time_diff.num_hours())
            } else {
                format!("{} days ago", time_diff.num_days())
            };
            
            RecentActivity {
                status: log.status.unwrap_or("unknown".to_string()).to_lowercase(),
                email: log.to_email,
                time: time_str,
            }
        }).collect();

        Ok(service_response(
            200,
            "Recent activity retrieved successfully",
            true,
            Some(serde_json::to_value(activities).unwrap()),
        ))
    }

    pub async fn get_system_status() -> Result<HttpResponse, AppError> {
        let status = SystemStatus {
            api_status: "All systems operational".to_string(),
            email_delivery: "99.9% uptime".to_string(),
            webhook_processing: "Minor delays (~2min)".to_string(),
        };

        Ok(service_response(
            200,
            "System status retrieved successfully",
            true,
            Some(serde_json::to_value(status).unwrap()),
        ))
    }
}