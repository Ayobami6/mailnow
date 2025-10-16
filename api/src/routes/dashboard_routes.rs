use crate::controllers::dashboard_controller::DashboardController;
use actix_web::web::{self, ServiceConfig};

pub fn register_dashboard_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/dashboard")
            .route("/stats", web::get().to(DashboardController::get_stats))
            .route("/activity", web::get().to(DashboardController::get_recent_activity))
            .route("/status", web::get().to(DashboardController::get_system_status)),
    );
}