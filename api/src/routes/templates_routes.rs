use crate::controllers::templates_controller::TemplatesController;
use actix_web::web::{self, ServiceConfig};

pub fn register_templates_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/templates")
            .route("", web::get().to(TemplatesController::get_templates))
            .route("", web::post().to(TemplatesController::create_template))
            .route("/stats", web::get().to(TemplatesController::get_template_stats))
            .route("/{template_id}", web::get().to(TemplatesController::get_template))
            .route("/{template_id}", web::put().to(TemplatesController::update_template))
            .route("/{template_id}", web::delete().to(TemplatesController::delete_template)),
    );
}