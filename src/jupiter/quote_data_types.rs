//! Data types for jupiter quote request api.

use serde::{Deserialize, Serialize};
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;

#[derive(Debug, Serialize, Deserialize)]
pub struct SwapInfo {
    pub ammKey: String,
    pub label: String,
    pub inputMint: String,
    pub outputMint: String,
    pub inAmount: String, // Changed to String since amounts can be large
    pub outAmount: String,
    pub feeAmount: String,
    pub feeMint: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoutePlan {
    pub swapInfo: SwapInfo,
    pub percent: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SwapMode {
    ExactIn,
    ExactOut,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuoteResponse {
    pub inputMint: String,
    pub inAmount: String,
    pub outputMint: String,
    pub outAmount: String,
    pub otherAmountThreshold: String,
    pub swapMode: SwapMode,
    pub slippageBps: u16,
    pub platformFee: Option<String>,
    pub priceImpactPct: String,
    pub routePlan: Vec<RoutePlan>,
    pub scoreReport: Option<String>, // New field
    pub contextSlot: u64,
    pub timeTaken: f64,
    pub swapUsdValue: Option<String>, // New field
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionRequest {
    pub userPublicKey: String,
    pub wrapAndUnwrapSol: bool,
    pub useSharedAccounts: bool,
    pub feeAccount: String,
    pub trackingAccount: String,
    pub prioritizationFeeLamports: u64,
    pub asLegacyTransaction: bool,
    pub useTokenLedger: bool,
    pub destinationTokenAccount: String,
    pub dynamicComputeUnitLimit: bool,
    pub skipUserAccountsRpcCalls: bool,
    pub dynamicSlippage: bool,
    pub computeUnitPriceMicroLamports: u64,
    pub quoteResponse: String,
}
