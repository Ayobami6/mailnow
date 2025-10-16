use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use crate::utils::utils::service_response;
use crate::errors::AppError;

#[derive(Serialize)]
pub struct EmailLog {
    pub id: String,
    pub timestamp: String,
    pub event: String,
    pub recipient: String,
    pub subject: String,
    pub status: String,
    pub message_id: String,
    pub response_time: String,
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
    pub async fn get_logs(query: web::Query<LogFilters>) -> Result<HttpResponse, AppError> {
        let logs = vec![
            EmailLog {
                id: "log_001".to_string(),
                timestamp: "2024-01-20 14:32:15".to_string(),
                event: "email.delivered".to_string(),
                recipient: "user@example.com".to_string(),
                subject: "Welcome to our platform".to_string(),
                status: "success".to_string(),
                message_id: "msg_abc123".to_string(),
                response_time: "245ms".to_string(),
            },
            EmailLog {
                id: "log_002".to_string(),
                timestamp: "2024-01-20 14:31:42".to_string(),
                event: "email.opened".to_string(),
                recipient: "customer@company.com".to_string(),
                subject: "Password reset request".to_string(),
                status: "success".to_string(),
                message_id: "msg_def456".to_string(),
                response_time: "189ms".to_string(),
            },
        ];

        Ok(service_response(
            200,
            "Email logs retrieved successfully",
            true,
            Some(serde_json::to_value(logs).unwrap()),
        ))
    }

    pub async fn get_log_stats() -> Result<HttpResponse, AppError> {
        let stats = LogStats {
            total_events: 12847,
            success_rate: 98.2,
            avg_response_time: "245ms".to_string(),
            failed_events: 231,
        };

        Ok(service_response(
            200,
            "Log stats retrieved successfully",
            true,
            Some(serde_json::to_value(stats).unwrap()),
        ))
    }

    pub async fn get_event_distribution() -> Result<HttpResponse, AppError> {
        let distribution = vec![
            EventDistribution {
                event: "email.delivered".to_string(),
                count: 8247,
                percentage: 64.2,
            },
            EventDistribution {
                event: "email.opened".to_string(),
                count: 2156,
                percentage: 16.8,
            },
            EventDistribution {
                event: "email.clicked".to_string(),
                count: 1247,
                percentage: 9.7,
            },
        ];

        Ok(service_response(
            200,
            "Event distribution retrieved successfully",
            true,
            Some(serde_json::to_value(distribution).unwrap()),
        ))
    }
}