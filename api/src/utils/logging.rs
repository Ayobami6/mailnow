use log::{info, warn, error};
use std::time::Instant;

pub struct RequestTimer {
    start: Instant,
    operation: String,
}

impl RequestTimer {
    pub fn new(operation: &str) -> Self {
        info!("Starting operation: {}", operation);
        Self {
            start: Instant::now(),
            operation: operation.to_string(),
        }
    }
}

impl Drop for RequestTimer {
    fn drop(&mut self) {
        let duration = self.start.elapsed();
        info!("Operation '{}' completed in {:?}", self.operation, duration);
        
        if duration.as_millis() > 1000 {
            warn!("Slow operation detected: '{}' took {:?}", self.operation, duration);
        }
    }
}

pub fn log_request_start(method: &str, path: &str, user_id: Option<i64>) {
    match user_id {
        Some(id) => info!("Request: {} {} (User: {})", method, path, id),
        None => info!("Request: {} {} (Anonymous)", method, path),
    }
}

pub fn log_database_operation(operation: &str, table: &str, affected_rows: Option<usize>) {
    match affected_rows {
        Some(rows) => info!("DB {}: {} ({} rows)", operation, table, rows),
        None => info!("DB {}: {}", operation, table),
    }
}

pub fn log_auth_event(event: &str, user_email: &str, success: bool) {
    if success {
        info!("Auth success: {} for {}", event, user_email);
    } else {
        warn!("Auth failure: {} for {}", event, user_email);
    }
}