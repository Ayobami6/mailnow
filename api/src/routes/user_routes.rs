use crate::controllers::user_controller::UserController;
use crate::middleware::auth::jwt_validator;
use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;

pub fn register_user_routes(cfg: &mut web::ServiceConfig) {
    let auth = HttpAuthentication::bearer(jwt_validator);
    cfg.service(
        web::scope("/user")
            .wrap(auth)
            .route("/profile", web::get().to(UserController::get_profile))
    );
}