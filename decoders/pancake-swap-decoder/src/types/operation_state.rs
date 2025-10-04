

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct OperationState {
    pub bump: u8,
    pub operation_owners: [solana_pubkey::Pubkey; 10],
    #[serde(with = "serde_big_array::BigArray")]
    pub whitelist_mints: [solana_pubkey::Pubkey; 100],
}
