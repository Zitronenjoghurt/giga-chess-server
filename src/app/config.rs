use std::sync::Arc;

pub struct Config {
    pub jwt_key: String,
}

impl Config {
    pub fn initialize() -> Arc<Self> {
        Arc::new(Self {
            jwt_key: std::env::var("JWT_KEY").unwrap(),
        })
    }
}
