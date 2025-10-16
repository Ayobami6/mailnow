use redis::{Client, Connection, RedisResult};
use std::env;

pub fn connect_redis() -> RedisResult<Client> {
    let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
    Client::open(redis_url)
}

pub async fn get_redis_connection() -> RedisResult<Connection> {
    let client = connect_redis()?;
    client.get_connection()
}