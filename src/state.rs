use {
    borsh::{BorshDeserialize, BorshSchema, BorshSerialize},
    solana_program::pubkey::Pubkey,
};

pub const LSD_LEN: usize = 136;

// #[derive(BorshSchema, BorshSerialize, BorshDeserialize)]
// struct YieldTokenWrapper {
//     underlying_lsu_mint: Pubkey,
//     underlying_lsu_amount: u128,
//     associated_pt: Pubkey,
//     associated_pt_amount: u128,
//     redemption_value_at_start: u128,
//     yield_claimed: u128,
//     maturity_date: i64,
// }

// #[derive(BorshSchema, BorshSerialize, BorshDeserialize)]
// struct PrincipalTokenWrapper {
//     underlying_lsu_mint: Pubkey,
//     underlying_lsu_amount: Pubkey,
//     associated_yt: Pubkey,
//     associated_yt_amount: u128,
//     redemption_value_at_start: u128,
//     maturity_date: i64,
// }

#[derive(BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct YieldTokenizerState {
    pub pt: Pubkey,
    pub yt: Pubkey,
    pub maturity_date: i64,
    pub lsu_mint: Pubkey,
    pub lsu_vault: Pubkey,
}
