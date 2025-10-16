use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::utils::utils::service_response;
use crate::errors::AppError;

#[derive(Serialize)]
pub struct EmailTemplate {
    pub id: String,
    pub name: String,
    pub subject: String,
    pub template_type: String,
    pub content: Option<String>,
    pub last_modified: String,
    pub usage: i64,
    pub status: String,
}

#[derive(Serialize)]
pub struct TemplateStats {
    pub total_templates: i32,
    pub active_templates: i32,
    pub total_usage: i64,
    pub this_month: i64,
}

#[derive(Deserialize)]
pub struct CreateTemplateRequest {
    pub name: String,
    pub subject: String,
    pub template_type: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct UpdateTemplateRequest {
    pub name: Option<String>,
    pub subject: Option<String>,
    pub content: Option<String>,
    pub status: Option<String>,
}

pub struct TemplatesController;

impl TemplatesController {
    pub async fn get_templates() -> Result<HttpResponse, AppError> {
        let templates = vec![
            EmailTemplate {
                id: "1".to_string(),
                name: "Welcome Email".to_string(),
                subject: "Welcome to {{company_name}}!".to_string(),
                template_type: "Transactional".to_string(),
                content: Some("<h1>Welcome!</h1><p>Thanks for joining us.</p>".to_string()),
                last_modified: "2 days ago".to_string(),
                usage: 1247,
                status: "active".to_string(),
            },
            EmailTemplate {
                id: "2".to_string(),
                name: "Password Reset".to_string(),
                subject: "Reset your password".to_string(),
                template_type: "Transactional".to_string(),
                content: Some("<h1>Password Reset</h1><p>Click the link to reset.</p>".to_string()),
                last_modified: "1 week ago".to_string(),
                usage: 89,
                status: "active".to_string(),
            },
        ];

        Ok(service_response(
            200,
            "Templates retrieved successfully",
            true,
            Some(serde_json::to_value(templates).unwrap()),
        ))
    }

    pub async fn get_template_stats() -> Result<HttpResponse, AppError> {
        let stats = TemplateStats {
            total_templates: 12,
            active_templates: 9,
            total_usage: 3948,
            this_month: 1247,
        };

        Ok(service_response(
            200,
            "Template stats retrieved successfully",
            true,
            Some(serde_json::to_value(stats).unwrap()),
        ))
    }

    pub async fn create_template(req: web::Json<CreateTemplateRequest>) -> Result<HttpResponse, AppError> {
        if req.name.is_empty() || req.subject.is_empty() {
            return Err(AppError::Validation("Template name and subject are required".to_string()));
        }

        let new_template = EmailTemplate {
            id: Uuid::new_v4().to_string(),
            name: req.name.clone(),
            subject: req.subject.clone(),
            template_type: req.template_type.clone(),
            content: Some(req.content.clone()),
            last_modified: "Just now".to_string(),
            usage: 0,
            status: "active".to_string(),
        };

        Ok(service_response(
            201,
            "Template created successfully",
            true,
            Some(serde_json::to_value(new_template).unwrap()),
        ))
    }

    pub async fn get_template(path: web::Path<String>) -> Result<HttpResponse, AppError> {
        let template_id = path.into_inner();
        
        let template = EmailTemplate {
            id: template_id,
            name: "Welcome Email".to_string(),
            subject: "Welcome to {{company_name}}!".to_string(),
            template_type: "Transactional".to_string(),
            content: Some("<h1>Welcome!</h1><p>Thanks for joining us.</p>".to_string()),
            last_modified: "2 days ago".to_string(),
            usage: 1247,
            status: "active".to_string(),
        };

        Ok(service_response(
            200,
            "Template retrieved successfully",
            true,
            Some(serde_json::to_value(template).unwrap()),
        ))
    }

    pub async fn update_template(
        path: web::Path<String>,
        req: web::Json<UpdateTemplateRequest>,
    ) -> Result<HttpResponse, AppError> {
        let _template_id = path.into_inner();

        Ok(service_response(
            200,
            "Template updated successfully",
            true,
            None,
        ))
    }

    pub async fn delete_template(path: web::Path<String>) -> Result<HttpResponse, AppError> {
        let _template_id = path.into_inner();

        Ok(service_response(
            200,
            "Template deleted successfully",
            true,
            None,
        ))
    }
}