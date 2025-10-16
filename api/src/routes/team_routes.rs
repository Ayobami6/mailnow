use actix_web::web;
use crate::controllers::team_controller::TeamController;

pub fn register_team_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/team")
            .route("/invite", web::post().to(TeamController::invite_member))
            .route("/accept-invite", web::post().to(TeamController::accept_invite))
            .route("/{company_id}/members", web::get().to(TeamController::list_team_members))
            .route("/{company_id}/members/{member_id}", web::get().to(TeamController::get_team_member))
    );
}