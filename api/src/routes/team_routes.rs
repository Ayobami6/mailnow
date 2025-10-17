use actix_web::web;
use crate::controllers::team_controller::TeamController;
use actix_web_httpauth::middleware::HttpAuthentication;
use crate::middleware::auth::jwt_validator;

pub fn register_team_routes(cfg: &mut web::ServiceConfig) {
    let auth = HttpAuthentication::bearer(jwt_validator);
    cfg.service(
        web::scope("/team")
            .service(
                web::scope("/protected")
                    .wrap(auth)
                    .route("/invite", web::post().to(TeamController::invite_member))
                    .route("/{company_id}/members", web::get().to(TeamController::list_team_members))
                    .route("/{company_id}/members/{member_id}", web::get().to(TeamController::get_team_member))
            )
            .route("/accept-invite", web::post().to(TeamController::accept_invite))
    );
}