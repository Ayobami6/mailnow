use crate::errors::AppError;
use crate::models::users::NewEmailLog;
use crate::repositories::{users::UserRepository, RepositoryFactory};
use crate::services::email_service::EmailService;
use crate::utils::utils::service_response;
use actix_web::{web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct SendEmailRequest {
    pub from: String,
    pub to: String,
    pub subject: String,
    pub html: Option<String>,
    pub text: Option<String>,
    pub template_id: Option<i64>,
}

#[derive(Serialize)]
pub struct SendEmailResponse {
    pub message_id: String,
    pub status: String,
}

pub struct PublicEmailController;

impl PublicEmailController {
    pub async fn send_email(
        req: HttpRequest,
        email_req: web::Json<SendEmailRequest>,
        repo_factory: web::Data<RepositoryFactory>,
    ) -> Result<HttpResponse, AppError> {
        // Extract API key from header
        let api_key = req
            .headers()
            .get("X-API-Key")
            .and_then(|h| h.to_str().ok())
            .ok_or_else(|| AppError::Forbidden("Missing X-API-Key header".to_string()))?;

        let user_repo = repo_factory.create_user_repository();

        // Validate API key and get company info
        let api_key_data = user_repo
            .get_api_key_by_key(api_key)
            .map_err(|_| AppError::Validation("Invalid API key".to_string()))?;

        if !api_key_data.is_active {
            return Err(AppError::Validation("API key is inactive".to_string()));
        }

        // Get company and check credits
        let company = user_repo.get_company_by_id(api_key_data.company_id)?;
        if company.api_credits <= 0 {
            return Err(AppError::Validation("Insufficient API credits".to_string()));
        }

        // Get default SMTP profile for the company
        let smtp_profile = user_repo
            .get_default_smtp_profile(company.id)
            .map_err(|_| AppError::Validation("No default SMTP profile configured".to_string()))?;

        // Deduct API credit
        user_repo.deduct_api_credit(company.id)?;

        // Prepare email content (check for template first)
        let (content, subject, is_html) = if let Some(template_id) = email_req.template_id {
            let template = user_repo
                .get_template_by_id(template_id, company.id)
                .map_err(|_| AppError::Validation("Template not found".to_string()))?;

            (template.content, template.subject, true)
        } else {
            let empty_string = String::new();
            let content = email_req
                .html
                .as_ref()
                .unwrap_or(email_req.text.as_ref().unwrap_or(&empty_string));
            (
                content.clone(),
                email_req.subject.clone(),
                email_req.html.is_some(),
            )
        };

        // Generate message ID
        let message_id = format!("msg_{}", Uuid::new_v4().simple());

        // Create email log entry
        let new_log = NewEmailLog {
            from_email: email_req.from.clone(),
            to_email: email_req.to.clone(),
            subject: subject.clone(),
            body: content.clone(),
            status: Some("Queued".to_string()),
            created_at: chrono::Utc::now(),
            company_id: company.id,
        };

        let email_log = user_repo.create_email_log(new_log)?;

        // Clone data for background task
        let smtp_server = smtp_profile.smtp_server.clone();
        let smtp_username = smtp_profile.smtp_username.clone();
        let smtp_password = smtp_profile.smtp_password.clone();
        let smtp_port = smtp_profile.smtp_port;
        let from_email = email_req.from.clone();
        let to_email = email_req.to.clone();
        let email_subject = subject.clone();
        let email_content = content.clone();
        let log_id = email_log.id;
        let repo_factory_clone = repo_factory.clone();

        // Send email in background
        tokio::spawn(async move {
            let email_service = EmailService::new();

            let result = email_service
                .send_email(
                    &smtp_server,
                    &smtp_username,
                    &smtp_password,
                    &from_email,
                    Some(smtp_port as u16),
                    &to_email,
                    &email_subject,
                    &email_content,
                    is_html,
                )
                .await;

            // Update email log status
            let user_repo = repo_factory_clone.create_user_repository();
            let status = if result.is_ok() { "Success" } else { "Failed" };

            if let Err(e) = user_repo.update_email_log_status(log_id, status) {
                log::error!(
                    "Failed to update email log status for ID {}: {:?}",
                    log_id,
                    e
                );
            }

            if result.is_ok() {
                log::info!("Email sent successfully for log ID: {}", log_id);
            } else {
                log::error!(
                    "Failed to send email for log ID: {}: {:?}",
                    log_id,
                    result.err()
                );
            }
        });

        let response = SendEmailResponse {
            message_id,
            status: "queued".to_string(),
        };

        Ok(service_response(
            200,
            "Email queued successfully",
            true,
            Some(serde_json::to_value(response).unwrap()),
        ))
    }
}
