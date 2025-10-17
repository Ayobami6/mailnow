use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::utils::utils::service_response;
use crate::errors::AppError;
use crate::repositories::users::UserRepository;
use crate::models::users::NewApiKey;

#[derive(Serialize)]
pub struct ApiKeyResponse {
    pub id: i64,
    pub name: String,
    pub key: String,
    pub permissions: Option<String>,
    pub last_used: Option<String>,
    pub created: String,
    pub status: String,
}

#[derive(Serialize)]
pub struct ApiKeyStats {
    pub total_keys: i32,
    pub active_keys: i32,
    pub api_calls_today: i32,
    pub rate_limit: String,
}

#[derive(Deserialize)]
pub struct CreateApiKeyRequest {
    pub name: String,
    pub permissions: String,
    pub company_id: i64,
    pub user_id: i64,
}

pub struct ApiKeysController;

impl ApiKeysController {
    pub async fn get_api_keys(
        company_id: web::Path<i64>,
        repo_factory: web::Data<crate::repositories::RepositoryFactory>,
    ) -> Result<HttpResponse, AppError> {
        let user_repo = repo_factory.create_user_repository();
        let api_keys = user_repo.get_api_keys_by_company(company_id.into_inner())
            .map_err(|e| AppError::Database(e))?;

        let response_keys: Vec<ApiKeyResponse> = api_keys.into_iter().map(|key| {
            ApiKeyResponse {
                id: key.id,
                name: key.name,
                key: key.api_key,
                permissions: key.permission,
                last_used: key.last_used.map(|d| d.format("%Y-%m-%d %H:%M").to_string()),
                created: key.created_at.format("%Y-%m-%d").to_string(),
                status: if key.is_active { "active".to_string() } else { "inactive".to_string() },
            }
        }).collect();

        Ok(service_response(
            200,
            "API keys retrieved successfully",
            true,
            Some(serde_json::to_value(response_keys).unwrap()),
        ))
    }

    pub async fn get_api_key_stats(
        company_id: web::Path<i64>,
        repo_factory: web::Data<crate::repositories::RepositoryFactory>,
    ) -> Result<HttpResponse, AppError> {
        let user_repo = repo_factory.create_user_repository();
        let api_keys = user_repo.get_api_keys_by_company(company_id.into_inner())
            .map_err(|e| AppError::Database(e))?;

        let stats = ApiKeyStats {
            total_keys: api_keys.len() as i32,
            active_keys: api_keys.iter().filter(|k| k.is_active).count() as i32,
            api_calls_today: 1247, // This would come from usage tracking
            rate_limit: "10k/hr".to_string(),
        };

        Ok(service_response(
            200,
            "API key stats retrieved successfully",
            true,
            Some(serde_json::to_value(stats).unwrap()),
        ))
    }

    pub async fn create_api_key(
        req: web::Json<CreateApiKeyRequest>,
        repo_factory: web::Data<crate::repositories::RepositoryFactory>,
    ) -> Result<HttpResponse, AppError> {
        if req.name.is_empty() {
            return Err(AppError::Validation("API key name is required".to_string()));
        }

        let user_repo = repo_factory.create_user_repository();
        
        // Check user role - only Owner and Admin can create API keys
        let user_role = user_repo.get_user_role_in_company(req.user_id, req.company_id)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => AppError::Validation("User not found in company".to_string()),
                _ => AppError::Database(e),
            })?;
        
        if user_role != "Owner" && user_role != "Admin" {
            return Err(AppError::Validation("Only owners and admins can create API keys".to_string()));
        }

        let api_key_value = format!("mn_live_{}", Uuid::new_v4().simple());
        let new_api_key = NewApiKey {
            name: req.name.clone(),
            api_key: api_key_value.clone(),
            company_id: req.company_id,
            permission: Some(req.permissions.clone()),
            created_at: chrono::Utc::now(),
            is_active: true,
        };

        let created_key = user_repo.create_api_key(new_api_key)
            .map_err(|e| AppError::Database(e))?;

        let response = ApiKeyResponse {
            id: created_key.id,
            name: created_key.name,
            key: created_key.api_key,
            permissions: created_key.permission,
            last_used: None,
            created: created_key.created_at.format("%Y-%m-%d").to_string(),
            status: "active".to_string(),
        };

        Ok(service_response(
            201,
            "API key created successfully",
            true,
            Some(serde_json::to_value(response).unwrap()),
        ))
    }

    pub async fn delete_api_key(
        path: web::Path<(i64, i64, i64)>, // (company_id, key_id, user_id)
        repo_factory: web::Data<crate::repositories::RepositoryFactory>,
    ) -> Result<HttpResponse, AppError> {
        let (company_id, key_id, user_id) = path.into_inner();
        let user_repo = repo_factory.create_user_repository();
        
        // Check user role - only Owner and Admin can delete API keys
        let user_role = user_repo.get_user_role_in_company(user_id, company_id)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => AppError::Validation("User not found in company".to_string()),
                _ => AppError::Database(e),
            })?;
        
        if user_role != "Owner" && user_role != "Admin" {
            return Err(AppError::Validation("Only owners and admins can delete API keys".to_string()));
        }

        let deleted_count = user_repo.delete_api_key(key_id, company_id)
            .map_err(|e| AppError::Database(e))?;

        if deleted_count == 0 {
            return Err(AppError::Validation("API key not found".to_string()));
        }

        Ok(service_response(
            200,
            "API key deleted successfully",
            true,
            None,
        ))
    }
}