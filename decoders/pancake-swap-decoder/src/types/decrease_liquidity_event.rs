

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct DecreaseLiquidityEvent {
    pub position_nft_mint: solana_pubkey::Pubkey,
    pub liquidity: u128,
    pub decrease_amount_0: u64,
    pub decrease_amount_1: u64,
    pub fee_amount_0: u64,
    pub fee_amount_1: u64,
    pub reward_amounts: [u64; 3],
    pub transfer_fee_0: u64,
    pub transfer_fee_1: u64,
}
