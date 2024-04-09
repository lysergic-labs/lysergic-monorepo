use {
    borsh::{BorshDeserialize, BorshSchema, BorshSerialize},
    solana_program::{
        instruction::{AccountMeta, Instruction},
        msg,
        program_error::ProgramError,
        pubkey::Pubkey,
        system_program,
    },
    spl_associated_token_account::get_associated_token_address,
};

#[derive(Clone, Debug, PartialEq, Eq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub enum Expiry {
    TwelveMonths,
    EighteenMonths,
    TwentyFourMonths,
}

#[derive(Clone, Debug, PartialEq, Eq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub enum YieldTokenizerInstruction {
    /// Authority
    /// Yield Tokenizer Account
    /// LSU Token Mint
    /// Principal Token Mint
    /// Yield Mint
    /// LSU  Vault
    /// Token Program
    /// Assoc Token Program
    /// System Program
    InitializeYieldTokenizer { expiry: Expiry },

    /// Buyer
    /// Yield Tokenizer Account
    /// LSU Token Mint
    /// PT Mint
    /// YT Mint
    /// LSU Token Vault
    /// Buyer LSU ATA
    /// Buyer PT ATA
    /// Buyer YT ATA
    /// Token Program
    /// Assoc Token Program
    TokenizeYield { amount: u64 },

    /// Redeemer
    /// Yield Tokenizer Account
    /// LSU Token Mint
    /// PT Mint
    /// YT Mint
    /// LSU Token Vault
    /// Buyer LSU ATA
    /// Buyer PT ATA
    /// Buyer YT ATA
    /// Token Program
    /// Assoc Token Program
    Redeem { amount: u64 },

    /// Redeemer
    /// YieldTokenizerAccount
    /// LSU Token Mint
    /// PT Mint
    /// YT Mint
    /// LSU Token Vault
    /// Redeemer LSU ATA
    /// Redeemer PT ATA
    /// Redeemer YT ATA
    /// Token Program
    /// Assoc Token Program
    RedeemFromPt { amount: u64 },

    /// Claimer
    /// Yield Tokenizer Account
    /// LSU Token Mint
    /// YT Mint
    /// Claimer LSU ATA
    /// Claimer YT ATA
    /// Token Program
    /// Assoc Token Program
    ClaimYield,
}

/// Initialize a Yield Tokenizer for a specific maturity and liquid staking unit
pub fn init_yield_tokenizer(
    program_id: &Pubkey,
    authority: &Pubkey,
    yield_tokenizer: &Pubkey,
    lsu_mint: &Pubkey,
    pt_mint: &Pubkey,
    yt_mint: &Pubkey,
    lsu_vault: &Pubkey,
    expiry: Expiry,
) -> Result<Instruction, ProgramError> {
    Ok(Instruction::new_with_borsh(
        crate::id(),
        &YieldTokenizerInstruction::InitializeYieldTokenizer { expiry },
        vec![
            AccountMeta::new(*authority, true),
            AccountMeta::new(*yield_tokenizer, false),
            AccountMeta::new(*lsu_mint, false),
            AccountMeta::new(*pt_mint, false),
            AccountMeta::new(*yt_mint, false),
            AccountMeta::new(*lsu_vault, false),
            AccountMeta::new(spl_token::id(), false),
            AccountMeta::new(spl_associated_token_account::id(), false),
            AccountMeta::new(system_program::id(), false),
        ],
    ))
}

/// Tokenize a liquid staking unit into a principal token and a yield token
pub fn tokenize_yield(
    buyer: &Pubkey,
    yield_tokenizer: &Pubkey,
    lsu_mint: &Pubkey,
    pt_mint: &Pubkey,
    yt_mint: &Pubkey,
    lsu_vault: &Pubkey,
    buyer_lsu_ata: &Pubkey,
    buyer_pt_ata: &Pubkey,
    buyer_yt_ata: &Pubkey,
    amount: u64,
) -> Result<Instruction, ProgramError> {
    Ok(Instruction::new_with_borsh(
        crate::id(),
        &YieldTokenizerInstruction::TokenizeYield { amount },
        vec![
            AccountMeta::new(*buyer, true),
            AccountMeta::new(*yield_tokenizer, false),
            AccountMeta::new(*lsu_mint, false),
            AccountMeta::new(*pt_mint, false),
            AccountMeta::new(*yt_mint, false),
            AccountMeta::new(*lsu_vault, false),
            AccountMeta::new(*buyer_lsu_ata, false),
            AccountMeta::new(*buyer_pt_ata, false),
            AccountMeta::new(*buyer_yt_ata, false),
            AccountMeta::new(spl_token::id(), false),
            AccountMeta::new(spl_associated_token_account::id(), false),
        ],
    ))
}

/// Redeem a liquid staking unit from a principal token + yield token, the PT and YT must be in
/// 1:1 ratio
pub fn redeem(
    redeemer: &Pubkey,
    yield_tokenizer: &Pubkey,
    lsu_mint: &Pubkey,
    pt_mint: &Pubkey,
    yt_mint: &Pubkey,
    lsu_vault: &Pubkey,
    redeemer_lsu_ata: &Pubkey,
    redeemer_pt_ata: &Pubkey,
    redeemer_yt_ata: &Pubkey,
    amount: u64,
) -> Result<Instruction, ProgramError> {
    Ok(Instruction::new_with_borsh(
        crate::id(),
        &YieldTokenizerInstruction::Redeem { amount },
        vec![
            AccountMeta::new(*redeemer, true),
            AccountMeta::new(*yield_tokenizer, false),
            AccountMeta::new(*lsu_mint, false),
            AccountMeta::new(*pt_mint, false),
            AccountMeta::new(*yt_mint, false),
            AccountMeta::new(*lsu_vault, false),
            AccountMeta::new(*redeemer_lsu_ata, false),
            AccountMeta::new(*redeemer_pt_ata, false),
            AccountMeta::new(*redeemer_yt_ata, false),
            AccountMeta::new(spl_token::id(), false),
            AccountMeta::new(spl_associated_token_account::id(), false),
        ],
    ))
}

/// Redeem a liquid staking unit from a principal token only - can only be called after expiry
pub fn redeem_from_pt(
    redeemer: &Pubkey,
    yield_tokenizer: &Pubkey,
    lsu_mint: &Pubkey,
    pt_mint: &Pubkey,
    lsu_vault: &Pubkey,
    pt_vault: &Pubkey,
    redeemer_lsu_ata: &Pubkey,
    redeemer_pt_ata: &Pubkey,
    amount: u64,
) -> Result<Instruction, ProgramError> {
    Ok(Instruction::new_with_borsh(
        crate::id(),
        &YieldTokenizerInstruction::RedeemFromPt { amount },
        vec![
            AccountMeta::new(*redeemer, true),
            AccountMeta::new(*yield_tokenizer, false),
            AccountMeta::new(*lsu_mint, false),
            AccountMeta::new(*pt_mint, false),
            AccountMeta::new(*lsu_vault, false),
            AccountMeta::new(*redeemer_lsu_ata, false),
            AccountMeta::new(*redeemer_pt_ata, false),
            AccountMeta::new(spl_token::id(), false),
            AccountMeta::new(spl_associated_token_account::id(), false),
        ],
    ))
}

/// Collect yield owed from holding a yield token
pub fn claim_yield(
    claimer: &Pubkey,
    yield_tokenizer: &Pubkey,
    lsu_mint: &Pubkey,
    yt_mint: &Pubkey,
    lsu_vault: &Pubkey,
    redeemer_lsu_ata: &Pubkey,
    redeemer_yt_ata: &Pubkey,
) -> Result<Instruction, ProgramError> {
    Ok(Instruction::new_with_borsh(
        crate::id(),
        &YieldTokenizerInstruction::ClaimYield,
        vec![
            AccountMeta::new(*claimer, true),
            AccountMeta::new(*yield_tokenizer, false),
            AccountMeta::new(*lsu_mint, false),
            AccountMeta::new(*yt_mint, false),
            AccountMeta::new(*lsu_vault, false),
            AccountMeta::new(*redeemer_lsu_ata, false),
            AccountMeta::new(*redeemer_yt_ata, false),
        ],
    ))
}
