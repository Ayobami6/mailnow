use crate::controllers::api_keys_controller::ApiKeysController;
use actix_web::web::{self, ServiceConfig};

pub fn register_api_keys_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/api-keys")
            .route("", web::get().to(ApiKeysController::get_api_keys))
            .route("", web::post().to(ApiKeysController::create_api_key))
            .route("/stats", web::get().to(ApiKeysController::get_api_key_stats))
            .route("/{key_id}", web::delete().to(ApiKeysController::delete_api_key)),
    );
}