//! Data types for solsniffer api.

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json;
use solana_sdk::pubkey::Pubkey;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnifferTokenResponse {
    pub token_data: TokenData,
    pub token_info: TokenInfo,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenData {
    pub indicator_data: IndicatorData,
    pub token_overview: TokenOverview,
    pub address: String,
    pub deploy_time: DateTime<Utc>,
    pub externals: String, // Consider using serde_json::Value if structure varies
    pub liquidity_list: Vec<HashMap<String, LiquidityPool>>,
    pub market_cap: f64,
    pub owners_list: Vec<OwnerInfo>,
    pub score: u32,
    pub token_img: String,
    pub token_name: String,
    pub token_symbol: String,
    pub audit_risk: AuditRisk,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndicatorData {
    pub high: RiskLevel,
    pub moderate: RiskLevel,
    pub low: RiskLevel,
    pub specific: RiskLevel,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiskLevel {
    pub count: u32,
    #[serde(deserialize_with = "deserialize_details")]
    pub details: HashMap<String, bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenOverview {
    pub deployer: String,
    pub mint: String,
    pub address: String,
    #[serde(rename = "type")]
    pub token_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiquidityPool {
    pub address: String,
    pub amount: f64,
    pub lp_pair: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OwnerInfo {
    pub address: String,
    #[serde(deserialize_with = "string_to_f64")]
    pub amount: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub percentage: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditRisk {
    pub mint_disabled: bool,
    pub freeze_disabled: bool,
    pub lp_burned: bool,
    pub top10_holders: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenInfo {
    #[serde(deserialize_with = "string_to_f64")]
    pub price: f64,
    pub supply_amount: f64,
    #[serde(rename = "mktCap")]
    pub market_cap: f64,
}

// Custom deserialization functions
fn deserialize_details<'de, D>(deserializer: D) -> Result<HashMap<String, bool>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    serde_json::from_str(&s).map_err(serde::de::Error::custom)
}

fn string_to_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.parse::<f64>().map_err(serde::de::Error::custom)
}
