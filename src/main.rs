use dotenv::dotenv;
use std::env;
mod ascii_arts;

fn main() {
    dotenv().ok();

    let safe_browse_api_key = env::var("SAFE_BROWSE_API_KEY").expect("SAFE_BROWSE_API_KEY not set");

    println!("API_KEY: {}", safe_browse_api_key);
    println!("Title: {}", ascii_arts::TITLE);
}
