use std::env;

fn main() {
    dotenv::dotenv().ok();

    let token = env::var("APP_BEARER_TOKEN").expect("missing token");

    println!("Hello, world!");
}
