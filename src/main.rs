#![allow(non_snake_case, unused_imports)]
use base64::Engine;
use clap::Parser;
use dotenv::dotenv;
use env_logger::Builder;
use log::LevelFilter;
use reqwest::*;
use serde::Deserialize;
use serde_json::Value;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey,
    signer::{Signer, keypair::Keypair},
    transaction::VersionedTransaction,
};
use solswap::{
    common::{
        self,
        helpers::{
            self, Command, MEME_TOKEN, QuoteCommands, WSOL, decimalized_value, get_token_data,
        },
    },
    jupiter::{
        quote::quote_api_caller,
        quote_data_types::TransactionRequest,
        swap::{build_txn, sign_send_tx},
    },
};
use std::{io::Write, process::exit, sync::Arc};

#[tokio::main]
async fn main() {
    // Command line parse.
    let opts = Command::parse();
    // Env variables loader, reads the .env file.
    dotenv().ok();

    // logger config and initialize
    let mut builder = Builder::new();
    builder
        .format(|buf, record| {
            writeln!(
                buf,
                "{}: {}: {}",
                buf.timestamp(),
                record.level(),
                record.args()
            )
        })
        .filter_level(LevelFilter::Info)
        .init();

    // load keypair dev
    let devKP = Keypair::from_base58_string(&std::env::var("coldpk").unwrap());

    // establish rpc connection
    let con = Arc::new(RpcClient::new(
        "https://solana-mainnet.core.chainstack.com/8d701a8cf39221fedef455984ecd8b4f",
    ));

    // Http client already wrapped in Arc
    let client = reqwest::Client::new();

    // helpers pubkeys
    let wsol = WSOL.parse::<Pubkey>().unwrap();
    let meme_token = MEME_TOKEN.parse::<Pubkey>().unwrap();

    match opts {
        Command::QUOTE { subcmd } => {
            match subcmd {
                QuoteCommands::SolIn {
                    outputMint,
                    inAmount,
                } => {
                    log::info!("Sol using as input\n");

                    let quote = quote_api_caller(&con, true, wsol, outputMint, inAmount)
                        .await
                        .unwrap();
                    log::info!(
                        "OutputAmount while sol is input: {:?}\n",
                        decimalized_value(&con, &quote.outAmount, outputMint)
                            .await
                            .unwrap()
                    )
                }
                QuoteCommands::SolOut {
                    inputMint,
                    inAmount,
                } => {
                    log::info!("Sol using as output");
                    let quote = quote_api_caller(&con, false, inputMint, wsol, inAmount)
                        .await
                        .unwrap();
                    log::info!(
                        "OutputAmount while sol is output: {:?}\n",
                        decimalized_value(&con, &quote.outAmount, wsol)
                            .await
                            .unwrap()
                    );
                }
            };
        }
        Command::SWAP { subcmd } => {
            match subcmd {
                QuoteCommands::SolIn {
                    outputMint,
                    inAmount,
                } => {
                    log::info!("Swap executing using WSOL as input\n");
                    let serialized_quote = quote_api_caller(&con, true, wsol, outputMint, inAmount)
                        .await
                        .unwrap();
                    sign_send_tx(&client, devKP, serialized_quote)
                        .await
                        .unwrap();
                }
                QuoteCommands::SolOut {
                    inputMint,
                    inAmount,
                } => {
                    log::info!("Swap executing using WSOL as output\n");
                    let serialized_quote = quote_api_caller(&con, false, inputMint, wsol, inAmount)
                        .await
                        .unwrap();
                    sign_send_tx(&client, devKP, serialized_quote)
                        .await
                        .unwrap();
                }
            };
        }
    }
}
