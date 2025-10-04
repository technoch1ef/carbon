
use super::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct PoolState {
    pub bump: [u8; 1],
    pub amm_config: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub token_mint_0: solana_pubkey::Pubkey,
    pub token_mint_1: solana_pubkey::Pubkey,
    pub token_vault_0: solana_pubkey::Pubkey,
    pub token_vault_1: solana_pubkey::Pubkey,
    pub observation_key: solana_pubkey::Pubkey,
    pub mint_decimals_0: u8,
    pub mint_decimals_1: u8,
    pub tick_spacing: u16,
    pub liquidity: u128,
    pub sqrt_price_x64: u128,
    pub tick_current: i32,
    pub padding3: u16,
    pub padding4: u16,
    pub fee_growth_global_0_x64: u128,
    pub fee_growth_global_1_x64: u128,
    pub protocol_fees_token_0: u64,
    pub protocol_fees_token_1: u64,
    pub swap_in_amount_token_0: u128,
    pub swap_out_amount_token_1: u128,
    pub swap_in_amount_token_1: u128,
    pub swap_out_amount_token_0: u128,
    pub status: u8,
    pub padding: [u8; 7],
    pub reward_infos: [RewardInfo; 3],
    pub tick_array_bitmap: [u64; 16],
    pub total_fees_token_0: u64,
    pub total_fees_claimed_token_0: u64,
    pub total_fees_token_1: u64,
    pub total_fees_claimed_token_1: u64,
    pub fund_fees_token_0: u64,
    pub fund_fees_token_1: u64,
    pub open_time: u64,
    pub recent_epoch: u64,
    pub padding1: [u64; 24],
    pub padding2: [u64; 32],
}
