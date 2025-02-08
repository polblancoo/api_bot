use reqwest::Client;
use std::env;

pub async fn fetch_coingecko_data() -> Result<String, reqwest::Error> {
    let api_key = env::var("COINGECKO_API_KEY").expect("COINGECKO_API_KEY must be set");
    let client = Client::new();
    let response = client
        .get("https://api.coingecko.com/api/v3/ping")
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await?;
    let body = response.text().await?;
    Ok(body)
} 