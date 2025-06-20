use std::env::var;
use std::sync::Arc;

pub struct Config {
    pub jwt_token_ttl_sec: u64,
    pub jwt_key: String,
    pub room_creation_limit: usize,
    pub user_cache_capacity: u64,
    pub user_cache_ttl_sec: u64,
}

impl Config {
    pub fn initialize() -> Arc<Self> {
        let jwt_token_ttl_sec = var("JWT_TOKEN_TTL_SEC").unwrap().parse().unwrap();
        let jwt_key = var("JWT_KEY").unwrap();
        let room_creation_limit = var("ROOM_CREATION_LIMIT").unwrap().parse().unwrap();
        let user_cache_capacity = var("USER_CACHE_CAPACITY").unwrap().parse().unwrap();
        let user_cache_ttl_sec = var("USER_CACHE_TTL_SEC").unwrap().parse().unwrap();

        Arc::new(Self {
            jwt_token_ttl_sec,
            jwt_key,
            room_creation_limit,
            user_cache_capacity,
            user_cache_ttl_sec,
        })
    }
}
