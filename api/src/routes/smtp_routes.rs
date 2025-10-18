use crate::controllers::smtp_controller::SmtpController;
use crate::middleware::auth::jwt_validator;
use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;

pub fn register_smtp_routes(cfg: &mut web::ServiceConfig) {
    let auth = HttpAuthentication::bearer(jwt_validator);
    cfg.service(
        web::scope("/smtp-profiles")
            .wrap(auth)
            .route("", web::get().to(SmtpController::get_smtp_profiles))
            .route("", web::post().to(SmtpController::create_smtp_profile))
            .route("/{id}", web::put().to(SmtpController::update_smtp_profile))
            .route("/{id}", web::delete().to(SmtpController::delete_smtp_profile))
            .route("/{id}/set-default", web::patch().to(SmtpController::set_default_smtp_profile))
    );
}