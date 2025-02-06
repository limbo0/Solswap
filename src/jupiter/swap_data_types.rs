//! Data types to communicate to jup swap api.

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SwapResponse {
    pub swapTransaction: String,
    pub lastValidBlockHeight: u64,
    pub prioritizationFeeLamports: u64,
    pub computeUnitLimit: u64,
    pub prioritizationType: PrioritizationType,
    pub simulationSlot: Option<u64>,
    pub dynamicSlippageReport: Option<DynamicSlippageReport>,
    pub simulationError: Option<SimulationError>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrioritizationType {
    pub computeBudget: ComputeBudget,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ComputeBudget {
    pub microLamports: u64,
    pub estimatedMicroLamports: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DynamicSlippageReport {
    slippageBps: u64,
    otherAmount: Option<u64>,
    simulatedIncurredSlippageBps: Option<i32>,
    amplificationRatio: Option<f64>,
    categoryName: String,
    heuristicMaxSlippageBps: Option<u64>,
    rtseSlippageBps: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationError {
    pub errorCode: String,
    pub error: String,
}
