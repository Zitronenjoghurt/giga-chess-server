use std::sync::Arc;

pub struct Config {
    pub jwt_key: String,
    pub room_creation_limit: usize,
}

impl Config {
    pub fn initialize() -> Arc<Self> {
        Arc::new(Self {
            jwt_key: std::env::var("JWT_KEY").unwrap(),
            room_creation_limit: std::env::var("ROOM_CREATION_LIMIT")
                .unwrap()
                .parse()
                .unwrap(),
        })
    }
}
