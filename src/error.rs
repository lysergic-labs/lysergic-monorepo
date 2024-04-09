use {
    num_derive::FromPrimitive,
    num_traits::FromPrimitive as FromPrimitiveTrait,
    solana_program::{
        decode_error::DecodeError,
        msg,
        program_error::{PrintProgramError, ProgramError},
    },
    thiserror::Error,
};

#[derive(Clone, Debug, PartialEq, Eq, Error, FromPrimitive)]
pub enum YieldTokenizerError {
    #[error("Invalid Instruction")]
    InvalidInstruction,
    #[error("Invalid liquid staking unit")]
    InvalidLSU,
    #[error("LSU Token Account Mismatch")]
    LSUTokenAccountMismatch,
    #[error("Invalid yield tokenizer address")]
    InvalidYieldTokenizerAddress,
    #[error("Invalid principal token address")]
    InvalidPrincipalToken,
    #[error("Invalid yield token address")]
    InvalidYieldToken,
    #[error("Invalid vault address")]
    InvalidVault,
    #[error("Expired")]
    Expired,
}

impl From<YieldTokenizerError> for ProgramError {
    fn from(e: YieldTokenizerError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for YieldTokenizerError {
    fn type_of() -> &'static str {
        "Yield tokenizer error"
    }
}

impl PrintProgramError for YieldTokenizerError {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + PrintProgramError + FromPrimitiveTrait,
    {
        match self {
            YieldTokenizerError::InvalidInstruction => msg!("Invalid instruction"),
            YieldTokenizerError::InvalidLSU => msg!("Invalid liquid staking unit mint"),
            YieldTokenizerError::LSUTokenAccountMismatch => msg!("LSU Token account mismatch"),
            YieldTokenizerError::InvalidYieldTokenizerAddress => {
                msg!("Invalid yield tokenizer address")
            }
            YieldTokenizerError::InvalidPrincipalToken => msg!("Invalid principal token address"),
            YieldTokenizerError::InvalidYieldToken => msg!("Invalid yield token address"),
            YieldTokenizerError::Expired => msg!("Instrument has expired"),
            YieldTokenizerError::InvalidVault => {
                msg!("The provided LSU vault address is incorrect")
            }
        }
    }
}
