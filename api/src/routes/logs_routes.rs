use crate::controllers::logs_controller::LogsController;
use crate::middleware::auth::jwt_validator;
use actix_web::web::{self, ServiceConfig};
use actix_web_httpauth::middleware::HttpAuthentication;

pub fn register_logs_routes(cfg: &mut ServiceConfig) {
    let auth = HttpAuthentication::bearer(jwt_validator);
    cfg.service(
        web::scope("/logs")
            .wrap(auth)
            .route("", web::get().to(LogsController::get_logs))
            .route("/stats", web::get().to(LogsController::get_log_stats))
            .route("/distribution", web::get().to(LogsController::get_event_distribution)),
    );
}