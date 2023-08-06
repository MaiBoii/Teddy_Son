use reqwest;
use serde::{Deserialize, Serialize};
use dotenv::dotenv;

struct VirusTotalResponse {
    scan_id: String,
    scan_date: String,
    permalink: String,
}

pub fn check_url_safety(url: &str) -> Result<(), reqwest::Error> {
    dotenv().ok();
    let virustotal_api_key = env::var("VIRUSTOTAL_API_KEY").expect("VIRUSTOTAL_API_KEY not set");

    let url = format!("https://www.virustotal.com/api/v3/urls/{}", url);
    let client = reqwest::blocking::Client::new();
    let response: VirusTotalResponse = client
        .get(&url)
        .header("x-apikey", VIRUSTOTAL_API_KEY)
        .send()?
        .json()?;

    println!("Scan ID: {}", response.scan_id);
    println!("Scan Date: {}", response.scan_date);
    println!("Permalink: {}", response.permalink);

    Ok(())
}

