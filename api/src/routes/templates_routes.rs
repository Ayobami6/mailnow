use crate::controllers::templates_controller::TemplatesController;
use actix_web::web::{self, ServiceConfig};
use actix_web_httpauth::middleware::HttpAuthentication;
use crate::middleware::auth::jwt_validator;

pub fn register_templates_routes(cfg: &mut ServiceConfig) {
    let auth = HttpAuthentication::bearer(jwt_validator);
    cfg.service(
        web::scope("/templates")
            .wrap(auth)
            .route("", web::get().to(TemplatesController::get_templates))
            .route("", web::post().to(TemplatesController::create_template))
            .route("/stats", web::get().to(TemplatesController::get_template_stats))
            .route("/{template_id}", web::get().to(TemplatesController::get_template))
            .route("/{template_id}", web::put().to(TemplatesController::update_template))
            .route("/{template_id}", web::delete().to(TemplatesController::delete_template)),
    );
}