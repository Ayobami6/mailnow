use crate::controllers::auth_controller::AuthController;
use actix_web::web::{self, ServiceConfig};

pub fn register_auth_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/signup", web::post().to(AuthController::signup))
            .route("/login", web::post().to(AuthController::login))
            .route(
                "/verify-email",
                web::post().to(AuthController::verify_email_send),
            )
            .route(
                "/verify-email",
                web::get().to(AuthController::verify_email_token),
            ),
    );
}
