use crate::config::redis::get_redis_connection;
use redis::{Commands, RedisResult};
use uuid::Uuid;

pub async fn store_verification_token(user_id: i64, token: &str) -> RedisResult<()> {
    let mut conn = get_redis_connection().await?;
    let key = format!("verify_token:{}", token);
    conn.set_ex(key, user_id, 86400)?; // 24 hours expiry
    Ok(())
}

pub async fn get_user_id_from_token(token: &str) -> RedisResult<Option<i64>> {
    let mut conn = get_redis_connection().await?;
    let key = format!("verify_token:{}", token);
    let user_id: RedisResult<i64> = conn.get(&key);
    match user_id {
        Ok(id) => Ok(Some(id)),
        Err(_) => Ok(None),
    }
}

pub async fn remove_verification_token(token: &str) -> RedisResult<()> {
    let mut conn = get_redis_connection().await?;
    let key = format!("verify_token:{}", token);
    conn.del(key)?;
    Ok(())
}

pub fn generate_verification_token() -> String {
    Uuid::new_v4().to_string()
}
