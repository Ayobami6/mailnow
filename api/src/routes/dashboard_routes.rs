use crate::controllers::dashboard_controller::DashboardController;
use actix_web::web::{self, ServiceConfig};
use actix_web_httpauth::middleware::HttpAuthentication;
use crate::middleware::auth::jwt_validator;

pub fn register_dashboard_routes(cfg: &mut ServiceConfig) {
    let auth = HttpAuthentication::bearer(jwt_validator);
    cfg.service(
        web::scope("/dashboard")
            .wrap(auth)
            .route("/stats", web::get().to(DashboardController::get_stats))
            .route("/activity", web::get().to(DashboardController::get_recent_activity))
            .route("/status", web::get().to(DashboardController::get_system_status)),
    );
}