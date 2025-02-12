//! Stream transactions and decode what's needed.

// Websocket connection to stream the transactions.
// Decode every single block, every single transactions
// Inspect the transactions and see if the address you are looking for is there.
// NOTE: Max size of solana transactions is 1232.
use anyhow::Result;
use futures_util::StreamExt;
use solana_client::nonblocking::pubsub_client::PubsubClient;
use solana_sdk::pubkey::Pubkey;
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::sync::mpsc::channel;

pub async fn watch_subscriptions(websocket_url: &str) -> Result<()> {
    // The `PubsubClient` must be `Arc`ed to share it across tasks.
    let pubsub_client = Arc::new(PubsubClient::new(websocket_url).await?);

    println!("streaming");
    let pubsub_client = Arc::clone(&pubsub_client);
    let filter = solana_client::rpc_config::RpcBlockSubscribeFilter::MentionsAccountOrProgram(
        "PERPHjGBqRHArX4DySjwM6UJHiR3sWAatqfdBS2qQJu".to_string(),
    );
    let (mut sub, unsub) = pubsub_client
        .program_subscribe(
            &"6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P"
                .parse::<Pubkey>()
                .unwrap(),
            None,
        )
        .await
        .unwrap();

    // Do something with the subscribed messages.
    // This loop will end once the main task unsubscribes.
    while let Some(data) = sub.next().await {
        println!("------------------------------------------------------------");
        println!("Result: {:#?}", data);
    }

    // This type hint is necessary to allow the `async move` block to use `?`.
    Ok::<_, anyhow::Error>(())
}

#[cfg(test)]
pub mod tests {

    use super::*;

    #[tokio::test]
    async fn test_init() {
        watch_subscriptions(
            "wss://solana-mainnet.core.chainstack.com/bea89a67f455d5890d5ce22c61148ac6",
        )
        .await
        .unwrap();
    }
}
