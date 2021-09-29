use thiserror::Error;

use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum UtilError {
    #[error("PublicKeyMismatch")]
    PublicKeyMismatch,
    #[error("InvalidMintAuthority")]
    InvalidMintAuthority,
    #[error("UninitializedAccount")]
    UninitializedAccount,
    #[error("IncorrectOwner")]
    IncorrectOwner,
    #[error("PublicKeysShouldBeUnique")]
    PublicKeysShouldBeUnique,
    #[error("StatementFalse")]
    StatementFalse,
    #[error("NotRentExempt")]
    NotRentExempt,
}

impl From<UtilError> for ProgramError {
    fn from(e: UtilError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

#[derive(Error, Debug, Copy, Clone)]
pub enum RouletteError {
    #[error("ActiveSpin")]
    ActiveSpin,
    #[error("Inactive")]
    Inactive,
    #[error("NumericalOverflow")]
    NumericalOverflow,
    #[error("AmountTooLarge")]
    AmountTooLarge,
    #[error("InvalidSlot")]
    InvalidSlot,
    #[error("SuspiciousTransaction")]
    SuspiciousTransaction,
}

impl From<RouletteError> for ProgramError {
    fn from(e: RouletteError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
