pub struct TwitterConfig {
    username: String,
    token: String,
}

impl TwitterConfig {
    pub fn new() -> Self {
        TwitterConfig {
            token: env::var("APP_BEARER_TOKEN").expect("missing token"),
            username: env::var("APP_BEARER_TOKEN").expect("missing username"),
        }
    }
}
