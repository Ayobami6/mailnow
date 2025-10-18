use crate::controllers::public_email_controller::PublicEmailController;
use actix_web::web;

pub fn register_public_email_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1")
            .route("/email/send", web::post().to(PublicEmailController::send_email))
    );
}