use crate::auth::jwt::Claims;
use crate::errors::AppError;
use crate::repositories::{users::UserRepository, RepositoryFactory};
use crate::utils::utils::service_response;
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct EmailLogResponse {
    pub id: i64,
    pub timestamp: String,
    pub event: String,
    pub recipient: String,
    pub subject: String,
    pub status: String,
    pub from_email: String,
}

#[derive(Serialize)]
pub struct LogStats {
    pub total_events: i64,
    pub success_rate: f64,
    pub avg_response_time: String,
    pub failed_events: i64,
}

#[derive(Serialize)]
pub struct EventDistribution {
    pub event: String,
    pub count: i64,
    pub percentage: f64,
}

#[derive(Deserialize)]
pub struct LogFilters {
    pub time_range: Option<String>,
    pub event_type: Option<String>,
    pub status: Option<String>,
    pub search: Option<String>,
}

pub struct LogsController;

impl LogsController {
    pub async fn get_logs(
        claims: web::ReqData<Claims>,
        _query: web::Query<LogFilters>,
        repo_factory: web::Data<RepositoryFactory>,
    ) -> Result<HttpResponse, AppError> {
        let user_repo = repo_factory.create_user_repository();
        let user_id = claims.into_inner().user_id;

        // Get user's company through team membership
        let team_members = user_repo.get_team_members_by_user(user_id)?;
        let company_id = team_members
            .first()
            .ok_or_else(|| {
                AppError::Validation("User not associated with any company".to_string())
            })?
            .company_id;

        let email_logs = user_repo.get_email_logs_by_company(company_id)?;
        let logs: Vec<EmailLogResponse> = email_logs
            .into_iter()
            .map(|log| EmailLogResponse {
                id: log.id,
                timestamp: log.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
                event: "email.sent".to_string(),
                recipient: log.to_email,
                subject: log.subject,
                status: log.status.unwrap_or("Unknown".to_string()).to_lowercase(),
                from_email: log.from_email,
            })
            .collect();

        Ok(service_response(
            200,
            "Email logs retrieved successfully",
            true,
            Some(serde_json::to_value(logs).unwrap()),
        ))
    }

    pub async fn get_log_stats(
        claims: web::ReqData<Claims>,
        repo_factory: web::Data<RepositoryFactory>,
    ) -> Result<HttpResponse, AppError> {
        let user_repo = repo_factory.create_user_repository();
        let user_id = claims.into_inner().user_id;

        // Get user's company through team membership
        let team_members = user_repo.get_team_members_by_user(user_id)?;
        let company_id = team_members
            .first()
            .ok_or_else(|| {
                AppError::Validation("User not associated with any company".to_string())
            })?
            .company_id;

        let (total, sent, queued, failed) = user_repo.get_email_log_stats(company_id)?;
        let success_rate = if total > 0 {
            ((sent as f64 / total as f64) * 100.0 * 100.0).round() / 100.0
        } else {
            0.0
        };

        let stats = LogStats {
            total_events: total,
            success_rate,
            avg_response_time: "N/A".to_string(),
            failed_events: failed,
        };

        Ok(service_response(
            200,
            "Log stats retrieved successfully",
            true,
            Some(serde_json::to_value(stats).unwrap()),
        ))
    }

    pub async fn get_event_distribution(
        claims: web::ReqData<Claims>,
        repo_factory: web::Data<RepositoryFactory>,
    ) -> Result<HttpResponse, AppError> {
        let user_repo = repo_factory.create_user_repository();
        let user_id = claims.into_inner().user_id;

        // Get user's company through team membership
        let team_members = user_repo.get_team_members_by_user(user_id)?;
        let company_id = team_members
            .first()
            .ok_or_else(|| {
                AppError::Validation("User not associated with any company".to_string())
            })?
            .company_id;

        let (total, sent, queued, failed) = user_repo.get_email_log_stats(company_id)?;
        let mut distribution = Vec::new();

        if total > 0 {
            if sent > 0 {
                distribution.push(EventDistribution {
                    event: "Success".to_string(),
                    count: sent,
                    percentage: (((sent as f64 / total as f64) * 100.0) * 100.0).round() / 100.0,
                });
            }
            if queued > 0 {
                distribution.push(EventDistribution {
                    event: "Queued".to_string(),
                    count: queued,
                    percentage: (((queued as f64 / total as f64) * 100.0) * 100.0).round() / 100.0,
                });
            }
            if failed > 0 {
                distribution.push(EventDistribution {
                    event: "Failed".to_string(),
                    count: failed,
                    percentage: (((failed as f64 / total as f64) * 100.0) * 100.0).round() / 100.0,
                });
            }
        }

        Ok(service_response(
            200,
            "Event distribution retrieved successfully",
            true,
            Some(serde_json::to_value(distribution).unwrap()),
        ))
    }
}
