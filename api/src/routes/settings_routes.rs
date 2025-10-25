use crate::controllers::settings_controller::SettingsController;
use actix_web::web::{self, ServiceConfig};
use actix_web_httpauth::middleware::HttpAuthentication;
use crate::middleware::auth::jwt_validator;

pub fn register_settings_routes(cfg: &mut ServiceConfig) {
    let auth = HttpAuthentication::bearer(jwt_validator);
    cfg.service(
        web::scope("/settings")
            .wrap(auth)
            .route("/company", web::get().to(SettingsController::get_company_profile))
            .route("/company", web::put().to(SettingsController::update_company_profile))
            .route("/password", web::put().to(SettingsController::change_password)),
    );
}