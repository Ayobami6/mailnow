use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures_util::future::LocalBoxFuture;
use std::{
    future::{ready, Ready},
    time::Instant,
};

pub struct RequestLogging;

impl<S, B> Transform<S, ServiceRequest> for RequestLogging
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RequestLoggingMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RequestLoggingMiddleware { service }))
    }
}

pub struct RequestLoggingMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for RequestLoggingMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let start_time = Instant::now();
        let method = req.method().to_string();
        let path = req.path().to_string();
        let remote_addr = req.connection_info().realip_remote_addr().unwrap_or("unknown").to_string();
        
        // Extract user info if available
        let user_info = req.extensions().get::<crate::auth::jwt::Claims>()
            .map(|claims| format!("User:{}", claims.user_id))
            .unwrap_or_else(|| "Anonymous".to_string());

        log::info!("Request started: {} {} from {} ({})", method, path, remote_addr, user_info);

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            let duration = start_time.elapsed();
            let status = res.status().as_u16();
            
            if status >= 400 {
                log::warn!(
                    "Request completed: {} {} -> {} in {:?} ({})",
                    method, path, status, duration, user_info
                );
            } else {
                log::info!(
                    "Request completed: {} {} -> {} in {:?} ({})",
                    method, path, status, duration, user_info
                );
            }
            
            if duration.as_millis() > 1000 {
                log::warn!("Slow request detected: {} {} took {:?}", method, path, duration);
            }

            Ok(res)
        })
    }
}