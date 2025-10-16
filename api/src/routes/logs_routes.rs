use crate::controllers::logs_controller::LogsController;
use actix_web::web::{self, ServiceConfig};

pub fn register_logs_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/logs")
            .route("", web::get().to(LogsController::get_logs))
            .route("/stats", web::get().to(LogsController::get_log_stats))
            .route("/distribution", web::get().to(LogsController::get_event_distribution)),
    );
}