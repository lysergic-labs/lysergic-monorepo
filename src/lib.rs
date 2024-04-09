use solana_program::pubkey::Pubkey;

pub mod entrypoint;
pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;

pub(crate) const MSOL: &str = "mSoLzYCxHdYgdzU16g5QSh3i5K3z3KZK7ytfqcJm7So";
pub(crate) const JITOSOL: &str = "J1toso1uCk3RLmjorhTtrVwY9HJ7X8V9yYac6Y7kGCPn";
pub(crate) const BSOL: &str = "bSo13r4TkiE4KumL71LsHTPpL2euBYLFx6h9HP3piy1";
pub(crate) const JSOL: &str = "7Q2afV64in6N6SeZsAAB81TJzwDoD6zpqmHkzi9Dcavn";

pub(crate) const LSD_SEED: &[u8; 6] = b"___lsd";
pub(crate) const PT_SEED: &[u8; 5] = b"___bt";
pub(crate) const YT_SEED: &[u8; 5] = b"___yt";

solana_program::declare_id!("LSDjBzV1CdC4zeXETyLnoUddeBeQAvXXRo49j8rSguH");

pub fn get_yield_tokenizer_address(lsu_mint: &Pubkey, maturity_date: i64) -> Pubkey {
    let (yield_tokenizer_addr, _) = Pubkey::find_program_address(
        &[
            crate::LSD_SEED,
            lsu_mint.as_ref(),
            &maturity_date.to_le_bytes(),
        ],
        &crate::id(),
    );
    yield_tokenizer_addr
}

pub fn get_yield_token_address(
    yield_tokenizer: &Pubkey,
    lsu_mint: &Pubkey,
    maturity_date: i64,
) -> Pubkey {
    let (yield_token_addr, _) = Pubkey::find_program_address(
        &[
            yield_tokenizer.as_ref(),
            lsu_mint.as_ref(),
            &maturity_date.to_le_bytes(),
        ],
        &crate::id(),
    );

    yield_token_addr
}

pub fn get_principal_token_address(
    yield_tokenizer: &Pubkey,
    lsu_mint: &Pubkey,
    maturity_date: i64,
) -> Pubkey {
    let (principal_token_addr, _) = Pubkey::find_program_address(
        &[
            yield_tokenizer.as_ref(),
            lsu_mint.as_ref(),
            &maturity_date.to_le_bytes(),
        ],
        &crate::id(),
    );

    principal_token_addr
}
