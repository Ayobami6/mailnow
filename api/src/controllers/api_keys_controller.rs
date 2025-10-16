use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::utils::utils::service_response;
use crate::errors::AppError;

#[derive(Serialize)]
pub struct ApiKey {
    pub id: String,
    pub name: String,
    pub key: String,
    pub permissions: String,
    pub last_used: String,
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
}

pub struct ApiKeysController;

impl ApiKeysController {
    pub async fn get_api_keys() -> Result<HttpResponse, AppError> {
        let api_keys = vec![
            ApiKey {
                id: "1".to_string(),
                name: "Production API Key".to_string(),
                key: "mk_live_1234567890abcdef".to_string(),
                permissions: "Full Access".to_string(),
                last_used: "2 hours ago".to_string(),
                created: "Jan 15, 2024".to_string(),
                status: "active".to_string(),
            },
            ApiKey {
                id: "2".to_string(),
                name: "Development Key".to_string(),
                key: "mk_test_abcdef1234567890".to_string(),
                permissions: "Send Only".to_string(),
                last_used: "1 day ago".to_string(),
                created: "Jan 10, 2024".to_string(),
                status: "active".to_string(),
            },
        ];

        Ok(service_response(
            200,
            "API keys retrieved successfully",
            true,
            Some(serde_json::to_value(api_keys).unwrap()),
        ))
    }

    pub async fn get_api_key_stats() -> Result<HttpResponse, AppError> {
        let stats = ApiKeyStats {
            total_keys: 3,
            active_keys: 2,
            api_calls_today: 1247,
            rate_limit: "10k/hr".to_string(),
        };

        Ok(service_response(
            200,
            "API key stats retrieved successfully",
            true,
            Some(serde_json::to_value(stats).unwrap()),
        ))
    }

    pub async fn create_api_key(req: web::Json<CreateApiKeyRequest>) -> Result<HttpResponse, AppError> {
        if req.name.is_empty() {
            return Err(AppError::Validation("API key name is required".to_string()));
        }

        let new_key = ApiKey {
            id: Uuid::new_v4().to_string(),
            name: req.name.clone(),
            key: format!("mk_live_{}", Uuid::new_v4().simple()),
            permissions: req.permissions.clone(),
            last_used: "Never".to_string(),
            created: chrono::Utc::now().format("%b %d, %Y").to_string(),
            status: "active".to_string(),
        };

        Ok(service_response(
            201,
            "API key created successfully",
            true,
            Some(serde_json::to_value(new_key).unwrap()),
        ))
    }

    pub async fn delete_api_key(path: web::Path<String>) -> Result<HttpResponse, AppError> {
        let _key_id = path.into_inner();

        Ok(service_response(
            200,
            "API key deleted successfully",
            true,
            None,
        ))
    }
}