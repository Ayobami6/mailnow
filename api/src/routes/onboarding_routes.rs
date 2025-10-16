use crate::controllers::onboarding_controller::OnboardingController;
use actix_web::web::{self, ServiceConfig};

pub fn register_onboarding_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/onboarding")
            .route("/complete", web::post().to(OnboardingController::complete_onboarding)),
    );
}