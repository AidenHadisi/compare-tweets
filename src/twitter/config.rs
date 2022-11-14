use std::env;

pub struct TwitterConfig {
    pub username: String,
    pub token: String,
}

impl TwitterConfig {
    pub fn new() -> Self {
        TwitterConfig {
            token: env::var("APP_BEARER_TOKEN").expect("missing token"),
            username: env::var("APP_BEARER_TOKEN").expect("missing username"),
        }
    }
}
