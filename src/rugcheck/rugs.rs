use base64::{Engine, alphabet::STANDARD};
use chrono::{DateTime, Utc};
use reqwest::Client;
use solana_sdk::{signature::Keypair, signer::Signer};

pub async fn check_rugs(client: Client, Kp: Keypair) {
    // Create the message to be signed
    let message_data = serde_json::json!({
        "message": "Sign-in to Rugcheck.xyz".to_string(),
        "publicKey": Kp.pubkey().to_string(),
        "timestamp": Utc::now().timestamp(),
    });

    // Sign the message
    let message_bytes = message_data.to_string().as_bytes().to_vec();
    let signature = Kp.sign_message(&message_bytes);
    let enc = base64::engine::general_purpose::STANDARD.encode(signature);
    let dec = base64::engine::general_purpose::STANDARD
        .decode(enc)
        .unwrap()
        .to_vec();
    let signature_data = serde_json::json!({
        "data": &dec,
        "type": "ed25519",
    });

    // Create the request body
    let body = serde_json::json!({
        "signature": signature_data,
        "wallet": Kp.pubkey().to_string(),
        "message": message_data,
    });

    println!("{:#?}", &body);

    // Make the POST request
    let response = client
        .post("https://api.rugcheck.xyz/auth/login/solana")
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .unwrap();

    println!("{:?}", response);
}

#[cfg(test)]
pub mod tests {

    use super::*;
    use dotenv::dotenv;

    #[tokio::test]
    async fn test_rug() {
        dotenv().ok();
        let client = Client::new();
        let Kp = Keypair::from_base58_string(&std::env::var("pk").unwrap());
        check_rugs(client, Kp).await;
    }
}
