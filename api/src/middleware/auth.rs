use crate::auth::jwt::JwtService;
use actix_web::{dev::ServiceRequest, Error, HttpMessage};
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;

pub async fn jwt_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let config = Config::default().realm("Restricted area");

    let jwt_service = match req.app_data::<actix_web::web::Data<JwtService>>() {
        Some(service) => service,
        None => return Err((AuthenticationError::from(config).into(), req)),
    };

    match jwt_service.verify_token(credentials.token()) {
        Ok(claims) => {
            req.extensions_mut().insert(claims);
            Ok(req)
        }
        Err(_) => Err((AuthenticationError::from(config).into(), req)),
    }
}
