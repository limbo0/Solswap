//! Common helper functions and configs.
use anyhow::Result;
use clap::Parser;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{program_pack::Pack, pubkey::Pubkey, signature::Keypair};
use spl_token::state::Mint;
use std::sync::Arc;

use crate::Connection;

pub const PUMP_FUN_DECIMAL: i32 = 6;
pub const W_SOL_DECIMAL: i32 = 9;
pub const WSOL: &str = "So11111111111111111111111111111111111111112";
pub const JELLY: &str = "FeR8VBqNRSUD5NtXAj2n3j1dAHkZHfyDktKuLXD4pump";
pub const MEME_TOKEN: &str = "d4VVNcnhYwenMzJJk7QgZsxVbLQSijPSLB8m7gKMGFM";
pub const JITO_TX_API: &str = "https://mainnet.block-engine.jito.wtf/api/v1/transactions";

#[derive(Debug, Parser)]
pub enum QuoteCommands {
    SolIn {
        #[clap(long)]
        outputMint: Pubkey,
        #[clap(long)]
        inAmount: f64,
    },
    SolOut {
        #[clap(long)]
        inputMint: Pubkey,
        #[clap(long)]
        inAmount: f64,
    },
}
#[derive(Debug, Parser)]
#[command(name = "KhewaSwap")]
#[command(version = "1.0")]
#[command(about = "**********************************\n**********************************\n*****DOES AUTOMATED TRADE LOL*****\n**********************************\n**********************************\n", long_about = None)]
pub enum Command {
    SWAP {
        #[clap(subcommand)]
        subcmd: QuoteCommands,
    },
    QUOTE {
        #[clap(subcommand)]
        subcmd: QuoteCommands,
    },
}

pub async fn convert_pk() {
    let kp = Keypair::from_base58_string(
        "5eBuxWaAMRbqkbNrva7WP2mCPMHjuNGxEsEpFmhb53kYHB2vTLsWLFyr5bsRHw8bMiszKXMyBRycAKimguL2z35J",
    )
    .to_bytes();
    println!("{:?}", kp);
}

pub async fn get_token_data(con: &Connection, token: Pubkey) -> Result<Mint> {
    let token_account = con.get_account(&token).unwrap();
    let token_data = Mint::unpack_unchecked(&token_account.data).unwrap();
    Ok(token_data)
}

// Pass in the atomic value eg(1350000000), return the dciamlized value as per the tokens decimal.
pub async fn decimalized_value(
    con: &Connection,
    outAmount: &str,
    outputMInt: Pubkey,
) -> Result<f64> {
    if outputMInt == WSOL.parse::<Pubkey>().unwrap() {
        Ok((outAmount.parse::<f64>().unwrap()) / 10f64.powi(W_SOL_DECIMAL))
    } else {
        Ok((outAmount.parse::<f64>().unwrap())
            / 10f64.powi(
                get_token_data(con, outputMInt)
                    .await
                    .unwrap()
                    .decimals
                    .into(),
            ))
    }
}
