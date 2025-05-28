//! Jupiter swap api caller.

use crate::jupiter::{quote_data_types::QuoteResponse, swap_data_types::SwapResponse};
use crate::{ARpcCon, JitoTxResponse, common::helpers};
use anyhow::Result;
use base64::Engine;
use reqwest::{Client, Response, StatusCode};
use solana_sdk::{
    pubkey::Pubkey,
    signature::Signature,
    signer::{Signer, keypair::Keypair},
    transaction::VersionedTransaction,
};

// Calls the Jup swap api, returns sucessful swap response.
//
pub async fn build_txn(
    client: &Client,
    dev_public: Pubkey,
    serialized_quote: QuoteResponse,
) -> Result<SwapResponse, StatusCode> {
    log::info!("Building transaction!\n");
    let payload = serde_json::json!({
        "userPublicKey": dev_public.to_string(),
        "quoteResponse": serde_json::json!(serialized_quote),
        "dynamicSlippage": true,
    });
    let response = client
        .post("https://api.jup.ag/swap/v1/swap")
        .json(&payload)
        .send()
        .await
        .unwrap();
    if response.status().is_success() {
        let body = response.text().await.unwrap();
        log::info!("Raw swap response: {:#?}", body);
        let serialized_response: SwapResponse = serde_json::from_str(&body).unwrap();
        log::info!("serialized response: {:#?}\n", serialized_response);
        Ok(serialized_response)
    } else {
        log::error!("Error: {:?}\n", response.status());
        Err(response.status())
    }
}

// This method always sets skip_preflight=true,
// which means the transaction won’t be simulated in the RPC before being sent to the leader
//
pub async fn sign_send_tx(
    client: &Client,
    devKP: Keypair,
    serialized_quote: QuoteResponse,
) -> Result<String> {
    // Transaction is received as base64 encoded string.
    let swapTx = build_txn(client, devKP.pubkey(), serialized_quote)
        .await
        .unwrap()
        .swapTransaction;
    log::info!("checking here atm{:?}", swapTx);

    let decode = base64::engine::general_purpose::STANDARD
        .decode(swapTx)
        .unwrap();
    let ver_tx: VersionedTransaction = bincode::deserialize(&decode).unwrap();
    log::info!("Ver tx looking here {:?}", ver_tx);

    log::info!("Signing transaction!\n");
    let signed_tx = VersionedTransaction::try_new(ver_tx.message, &[devKP]).unwrap();

    // Final encoded transaction to send for execution.
    let encode_tx =
        base64::engine::general_purpose::STANDARD.encode(bincode::serialize(&signed_tx).unwrap());

    let payload = serde_json::json!(
    {
    "id": 1,
    "jsonrpc": "2.0",
    "method": "sendTransaction",
    "params": [
        encode_tx,
        {
        "encoding": "base64"
        }
    ]
    });

    let response = client
        .post(helpers::JITO_TX_API)
        .json(&payload)
        .send()
        .await
        .unwrap();

    if response.status().is_success() {
        let tx_id: JitoTxResponse = serde_json::from_str(&response.text().await.unwrap()).unwrap();
        log::info!("https://solscan.io/tx/{}\n", tx_id.result);
        Ok(tx_id.result)
    } else {
        log::info!("Jito server reponse erro: {:?}\n", response.status());
        Ok(format!(
            "Jito server reponse erro: {:?}\n",
            response.status()
        ))
    }
}
