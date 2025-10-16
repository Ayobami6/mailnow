use crate::controllers::email_controller::EmailController;
use actix_web::web::{self, ServiceConfig};

pub fn register_email_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/email")
            .route("/send", web::post().to(EmailController::send_email))
            .route("/recent", web::get().to(EmailController::get_recent_emails))
            .route("/status/{message_id}", web::get().to(EmailController::get_email_status)),
    );
}