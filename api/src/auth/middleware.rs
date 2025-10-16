use actix_web::{dev::ServiceRequest, Error, HttpMessage};
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;

use crate::auth::jwt::JwtService;

pub async fn jwt_middleware(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let jwt_service = req
        .app_data::<actix_web::web::Data<JwtService>>()
        .unwrap();

    match jwt_service.verify_token(credentials.token()) {
        Ok(claims) => {
            req.extensions_mut().insert(claims);
            Ok(req)
        }
        Err(_) => {
            let config = req
                .app_data::<Config>()
                .cloned()
                .unwrap_or_default();
            Err((AuthenticationError::from(config).into(), req))
        }
    }
}