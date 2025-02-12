//! Entrypoint for solsniffer api.
//! Sniffscore above 95: No risk detected.

use crate::solsniffer::data_types::SnifferTokenResponse;
use anyhow::Result;
use reqwest::{Client, header};
use solana_sdk::pubkey::Pubkey;
//
// Calls the api with header auth
pub async fn check_solsniff(sniffapi: String, token: Pubkey) -> Result<SnifferTokenResponse> {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        "X-API-KEY",
        header::HeaderValue::from_str(&sniffapi).unwrap(),
    );

    let response = Client::builder()
        .default_headers(headers)
        .build()
        .unwrap()
        .get(format!("https://solsniffer.com/api/v2/token/{token}"))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let serialized_response: SnifferTokenResponse = serde_json::from_str(&response).unwrap();
    Ok(serialized_response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;
    use tokio::test;

    #[test]
    async fn call_sniffer() {
        dotenv().ok();
        let sniffapi = std::env::var("solsniffer").unwrap();
        let token = "2sCUCJdVkmyXp4dT8sFaA9LKgSMK4yDPi9zLHiwXpump"
            .parse::<Pubkey>()
            .unwrap();
        let rd = check_solsniff(sniffapi, token).await.unwrap();
        println!("{:?}\n", rd.token_data.indicator_data.high.details);
        assert_eq!(1, rd.token_data.score);
    }
}
