use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use crate::utils::utils::service_response;
use crate::errors::AppError;

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
        company_id: web::Path<i64>,
        repo_factory: web::Data<crate::repositories::RepositoryFactory>,
    ) -> Result<HttpResponse, AppError> {
        use crate::middleware::credits::check_and_reset_credits;
        
        let company_id = company_id.into_inner();
        
        // Check and reset credits if needed
        check_and_reset_credits(company_id, &repo_factory).await?;
        
        let user_repo = repo_factory.create_user_repository();
        let company = user_repo.get_company_by_id(company_id)
            .map_err(|e| AppError::Database(e))?;
        
        let stats = DashboardStats {
            emails_sent: 12847,
            delivery_rate: 98.2,
            api_calls: 45231,
            active_users: 2847,
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

    pub async fn get_recent_activity() -> Result<HttpResponse, AppError> {
        let activities = vec![
            RecentActivity {
                status: "delivered".to_string(),
                email: "user@example.com".to_string(),
                time: "2 minutes ago".to_string(),
            },
            RecentActivity {
                status: "opened".to_string(),
                email: "customer@company.com".to_string(),
                time: "5 minutes ago".to_string(),
            },
            RecentActivity {
                status: "clicked".to_string(),
                email: "admin@startup.io".to_string(),
                time: "8 minutes ago".to_string(),
            },
            RecentActivity {
                status: "bounced".to_string(),
                email: "invalid@domain.com".to_string(),
                time: "12 minutes ago".to_string(),
            },
        ];

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