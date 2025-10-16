use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::utils::utils::service_response;
use crate::errors::AppError;

#[derive(Deserialize)]
pub struct SendEmailRequest {
    pub from: String,
    pub to: String,
    pub subject: String,
    pub html: Option<String>,
    pub text: Option<String>,
    pub template_id: Option<String>,
}

#[derive(Serialize)]
pub struct SendEmailResponse {
    pub message_id: String,
    pub status: String,
}

#[derive(Serialize)]
pub struct RecentEmail {
    pub to: String,
    pub subject: String,
    pub status: String,
    pub time: String,
    pub message_id: String,
}

pub struct EmailController;

impl EmailController {
    pub async fn send_email(req: web::Json<SendEmailRequest>) -> Result<HttpResponse, AppError> {
        if req.from.is_empty() || req.to.is_empty() || req.subject.is_empty() {
            return Err(AppError::Validation(
                "From, to, and subject are required".to_string(),
            ));
        }

        if req.html.is_none() && req.text.is_none() && req.template_id.is_none() {
            return Err(AppError::Validation(
                "Either HTML content, text content, or template ID is required".to_string(),
            ));
        }

        let message_id = format!("msg_{}", Uuid::new_v4().simple());
        
        let response = SendEmailResponse {
            message_id: message_id.clone(),
            status: "queued".to_string(),
        };

        log::info!("Email queued: {} -> {}", req.from, req.to);

        Ok(service_response(
            200,
            "Email sent successfully",
            true,
            Some(serde_json::to_value(response).unwrap()),
        ))
    }

    pub async fn get_recent_emails() -> Result<HttpResponse, AppError> {
        let recent_emails = vec![
            RecentEmail {
                to: "user@example.com".to_string(),
                subject: "Welcome to our platform".to_string(),
                status: "delivered".to_string(),
                time: "2 min ago".to_string(),
                message_id: "msg_abc123".to_string(),
            },
            RecentEmail {
                to: "customer@company.com".to_string(),
                subject: "Password reset request".to_string(),
                status: "opened".to_string(),
                time: "5 min ago".to_string(),
                message_id: "msg_def456".to_string(),
            },
            RecentEmail {
                to: "admin@startup.io".to_string(),
                subject: "Monthly newsletter".to_string(),
                status: "clicked".to_string(),
                time: "1 hour ago".to_string(),
                message_id: "msg_ghi789".to_string(),
            },
        ];

        Ok(service_response(
            200,
            "Recent emails retrieved successfully",
            true,
            Some(serde_json::to_value(recent_emails).unwrap()),
        ))
    }

    pub async fn get_email_status(path: web::Path<String>) -> Result<HttpResponse, AppError> {
        let message_id = path.into_inner();
        
        let status = serde_json::json!({
            "message_id": message_id,
            "status": "delivered",
            "delivered_at": "2024-01-20T14:32:15Z",
            "opened_at": "2024-01-20T14:35:22Z",
            "clicked_at": null
        });

        Ok(service_response(
            200,
            "Email status retrieved successfully",
            true,
            Some(status),
        ))
    }
}