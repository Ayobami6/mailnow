use crate::controllers::api_keys_controller::ApiKeysController;
use actix_web::web::{self, ServiceConfig};
use actix_web_httpauth::middleware::HttpAuthentication;
use crate::middleware::auth::jwt_validator;

pub fn register_api_keys_routes(cfg: &mut ServiceConfig) {
    let auth = HttpAuthentication::bearer(jwt_validator);
    cfg.service(
        web::scope("/api-keys")
            .wrap(auth)
            .route("/company/{company_id}", web::get().to(ApiKeysController::get_api_keys))
            .route("", web::post().to(ApiKeysController::create_api_key))
            .route("/company/{company_id}/stats", web::get().to(ApiKeysController::get_api_key_stats))
            .route("/company/{company_id}/key/{key_id}/user/{user_id}", web::delete().to(ApiKeysController::delete_api_key)),
    );
}