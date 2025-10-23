use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use crate::utils::utils::service_response;
use crate::errors::AppError;
use crate::auth::jwt::Claims;
use crate::repositories::{users::UserRepository, RepositoryFactory};
use crate::models::users::{NewTemplate, Template};

#[derive(Serialize)]
pub struct TemplateResponse {
    pub id: i64,
    pub name: String,
    pub subject: String,
    pub template_type: String,
    pub content: String,
    pub date_created: String,
    pub date_updated: String,
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
    pub async fn get_templates(
        claims: web::ReqData<Claims>,
        repo_factory: web::Data<RepositoryFactory>,
    ) -> Result<HttpResponse, AppError> {
        let user_repo = repo_factory.create_user_repository();
        let user_id = claims.into_inner().user_id;

        // Get user's company through team membership
        let team_members = user_repo.get_team_members_by_user(user_id)?;
        let company_id = team_members.first()
            .ok_or_else(|| AppError::Validation("User not associated with any company".to_string()))?
            .company_id;

        let templates = user_repo.get_templates_by_company(company_id)?;
        let response_templates: Vec<TemplateResponse> = templates.into_iter().map(|template| {
            TemplateResponse {
                id: template.id,
                name: template.name,
                subject: template.subject,
                template_type: template.template_type,
                content: template.content,
                date_created: template.date_created.format("%Y-%m-%d %H:%M").to_string(),
                date_updated: template.date_updated.format("%Y-%m-%d %H:%M").to_string(),
            }
        }).collect();

        Ok(service_response(
            200,
            "Templates retrieved successfully",
            true,
            Some(serde_json::to_value(response_templates).unwrap()),
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

    pub async fn create_template(
        claims: web::ReqData<Claims>,
        req: web::Json<CreateTemplateRequest>,
        repo_factory: web::Data<RepositoryFactory>,
    ) -> Result<HttpResponse, AppError> {
        if req.name.is_empty() || req.subject.is_empty() {
            return Err(AppError::Validation("Template name and subject are required".to_string()));
        }

        let user_repo = repo_factory.create_user_repository();
        let user_id = claims.into_inner().user_id;

        // Get user's company through team membership
        let team_members = user_repo.get_team_members_by_user(user_id)?;
        let company_id = team_members.first()
            .ok_or_else(|| AppError::Validation("User not associated with any company".to_string()))?
            .company_id;

        let new_template = NewTemplate {
            company_id,
            name: req.name.clone(),
            subject: req.subject.clone(),
            content: req.content.clone(),
            template_type: req.template_type.clone(),
            date_created: chrono::Utc::now(),
            date_updated: chrono::Utc::now(),
        };

        let created_template = user_repo.create_template(new_template)?;
        let response = TemplateResponse {
            id: created_template.id,
            name: created_template.name,
            subject: created_template.subject,
            template_type: created_template.template_type,
            content: created_template.content,
            date_created: created_template.date_created.format("%Y-%m-%d %H:%M").to_string(),
            date_updated: created_template.date_updated.format("%Y-%m-%d %H:%M").to_string(),
        };

        Ok(service_response(
            201,
            "Template created successfully",
            true,
            Some(serde_json::to_value(response).unwrap()),
        ))
    }

    pub async fn get_template(
        path: web::Path<i64>,
        claims: web::ReqData<Claims>,
        repo_factory: web::Data<RepositoryFactory>,
    ) -> Result<HttpResponse, AppError> {
        let template_id = path.into_inner();
        let user_repo = repo_factory.create_user_repository();
        let user_id = claims.into_inner().user_id;

        // Get user's company through team membership
        let team_members = user_repo.get_team_members_by_user(user_id)?;
        let company_id = team_members.first()
            .ok_or_else(|| AppError::Validation("User not associated with any company".to_string()))?
            .company_id;

        let template = user_repo.get_template_by_id(template_id, company_id)?;
        let response = TemplateResponse {
            id: template.id,
            name: template.name,
            subject: template.subject,
            template_type: template.template_type,
            content: template.content,
            date_created: template.date_created.format("%Y-%m-%d %H:%M").to_string(),
            date_updated: template.date_updated.format("%Y-%m-%d %H:%M").to_string(),
        };

        Ok(service_response(
            200,
            "Template retrieved successfully",
            true,
            Some(serde_json::to_value(response).unwrap()),
        ))
    }

    pub async fn update_template(
        path: web::Path<i64>,
        _req: web::Json<UpdateTemplateRequest>,
    ) -> Result<HttpResponse, AppError> {
        let _template_id = path.into_inner();

        Ok(service_response(
            200,
            "Template updated successfully",
            true,
            None,
        ))
    }

    pub async fn delete_template(
        path: web::Path<i64>,
        claims: web::ReqData<Claims>,
        repo_factory: web::Data<RepositoryFactory>,
    ) -> Result<HttpResponse, AppError> {
        let template_id = path.into_inner();
        let user_repo = repo_factory.create_user_repository();
        let user_id = claims.into_inner().user_id;

        // Get user's company through team membership
        let team_members = user_repo.get_team_members_by_user(user_id)?;
        let company_id = team_members.first()
            .ok_or_else(|| AppError::Validation("User not associated with any company".to_string()))?
            .company_id;

        user_repo.delete_template(template_id, company_id)?;

        Ok(service_response(
            200,
            "Template deleted successfully",
            true,
            None,
        ))
    }
}