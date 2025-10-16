use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

pub fn get_env(key: &str, fallback: &str) -> String {
    match std::env::var(key) {
        Ok(val) => val,
        Err(_) => fallback.to_string(),
    }
}

// create a reponse Object
// lets derive some traits for serialization and deserialization
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Response {
    pub status_code: i16,
    pub message: String,
    pub success: bool,
    pub data: Option<serde_json::Value>,
}

// Reponse object constructor in the implementations

impl Response {
    pub fn new(
        status_code: i16,
        message: &str,
        success: bool,
        data: Option<serde_json::Value>,
    ) -> Self {
        Response {
            status_code,
            message: message.to_string(),
            success,
            data,
        }
    }

    // to json
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "status_code": self.status_code,
            "message": self.message,
            "success": self.success,
            "data": self.data.clone().unwrap_or(serde_json::Value::Null),
        })
    }
}

// service_response function
pub fn service_response(
    status_code: i16,
    message: &str,
    success: bool,
    data: Option<serde_json::Value>,
) -> HttpResponse {
    let response = Response::new(status_code, message, success, data);

    match status_code {
        200 => HttpResponse::Ok().json(response.to_json()),
        201 => HttpResponse::Created().json(response.to_json()),
        202 => HttpResponse::Accepted().json(response.to_json()),
        400 => HttpResponse::BadRequest().json(response.to_json()),
        401 => HttpResponse::Unauthorized().json(response.to_json()),
        403 => HttpResponse::Forbidden().json(response.to_json()),
        404 => HttpResponse::NotFound().json(response.to_json()),
        422 => HttpResponse::UnprocessableEntity().json(response.to_json()),
        429 => HttpResponse::TooManyRequests().json(response.to_json()),
        500 => HttpResponse::InternalServerError().json(response.to_json()),
        409 => HttpResponse::Conflict().json(response.to_json()),
        503 => HttpResponse::ServiceUnavailable().json(response.to_json()),
        _ => HttpResponse::InternalServerError().json(response.to_json()),
    }
}
