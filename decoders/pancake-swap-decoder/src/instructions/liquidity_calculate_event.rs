

use carbon_core::{borsh, CarbonDeserialize};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xe445a52e51cb9a1ded7094e63954b4a2")]
pub struct LiquidityCalculateEvent{
    pub pool_liquidity: u128,
    pub pool_sqrt_price_x64: u128,
    pub pool_tick: i32,
    pub calc_amount_0: u64,
    pub calc_amount_1: u64,
    pub trade_fee_owed_0: u64,
    pub trade_fee_owed_1: u64,
    pub transfer_fee_0: u64,
    pub transfer_fee_1: u64,
}
