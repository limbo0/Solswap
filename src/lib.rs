#![allow(non_snake_case, unused_imports)]

pub mod common;
pub mod jupiter;
pub mod rugcheck;
pub mod solsniffer;
/*__________________________________________ */

use serde::{Deserialize, Serialize};
use solana_client::rpc_client::RpcClient;
use std::sync::Arc;

type ARpcCon = Arc<RpcClient>;

#[derive(Debug, Serialize, Deserialize)]
pub struct JitoTxResponse {
    pub jsonrpc: String,
    pub result: String,
    pub id: u16,
}
