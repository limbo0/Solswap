//! Jupiter quote api caller

use crate::{
    ARpcCon,
    common::helpers::{PUMP_FUN_DECIMAL, W_SOL_DECIMAL, get_token_data},
    jupiter::quote_data_types::QuoteResponse,
};
use anyhow::Result;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::sync::Arc;

// Passing u64 coz the input amount is not in atomic units.
//
pub async fn quote_api_caller(
    con: &ARpcCon,
    sol_input: bool,
    inputMint: Pubkey,
    outputMint: Pubkey,
    input_amount: f64,
) -> Result<QuoteResponse> {
    // change the decimal if not pumpfun token.
    if !sol_input {
        // input amount in atomic units in the context of decimal value of the input token decimal.
        let token_decimal = get_token_data(con, inputMint).await.unwrap().decimals as i32;
        let input_amount = input_amount * 10f64.powi(token_decimal);
        let response = reqwest::get(format!(
            "https://api.jup.ag/swap/v1/quote?inputMint={inputMint}\
                    &outputMint={outputMint}\
                    &amount={input_amount}\
                    &slippageBps=50",
        ))
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

        let serialized_response: QuoteResponse = serde_json::from_str(&response).unwrap();
        Ok(serialized_response)
    } else {
        let input_amount = (input_amount * 10f64.powi(W_SOL_DECIMAL)) as u64;
        // TODO: create dymanic slppage
        let response = reqwest::get(format!(
            "https://api.jup.ag/swap/v1/quote?inputMint={inputMint}\
                    &outputMint={outputMint}\
                    &amount={input_amount}\
                    &slippageBps=50",
        ))
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

        let serialized_response: QuoteResponse = serde_json::from_str(&response).unwrap();
        Ok(serialized_response)
    }
}
