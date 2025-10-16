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
    pub async fn get_stats() -> Result<HttpResponse, AppError> {
        let stats = DashboardStats {
            emails_sent: 12847,
            delivery_rate: 98.2,
            api_calls: 45231,
            active_users: 2847,
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