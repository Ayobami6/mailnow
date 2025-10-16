use std::collections::HashMap;
use std::sync::Mutex;
use chrono::{DateTime, Utc, Duration};

// Simple in-memory token storage (for production, use Redis or database)
lazy_static::lazy_static! {
    static ref VERIFICATION_TOKENS: Mutex<HashMap<String, VerificationToken>> = Mutex::new(HashMap::new());
}

#[derive(Clone)]
pub struct VerificationToken {
    pub email: String,
    pub expires_at: DateTime<Utc>,
}

pub fn store_verification_token(token: &str, email: &str) {
    let verification_token = VerificationToken {
        email: email.to_string(),
        expires_at: Utc::now() + Duration::hours(24), // 24 hour expiry
    };
    
    let mut tokens = VERIFICATION_TOKENS.lock().unwrap();
    tokens.insert(token.to_string(), verification_token);
}

pub fn get_verification_token(token: &str) -> Option<VerificationToken> {
    let mut tokens = VERIFICATION_TOKENS.lock().unwrap();
    
    if let Some(verification_token) = tokens.get(token) {
        // Check if token is expired
        if Utc::now() > verification_token.expires_at {
            tokens.remove(token);
            return None;
        }
        
        let token_data = verification_token.clone();
        tokens.remove(token); // Remove token after use (one-time use)
        Some(token_data)
    } else {
        None
    }
}