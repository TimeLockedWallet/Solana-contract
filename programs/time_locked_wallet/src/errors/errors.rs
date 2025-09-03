use anchor_lang::prelude::*;

#[error_code]

pub enum TimeLockedWalletError {
    #[msg("Cannot withdraw before unlock time.")]
    TimeLockNotExpired,

    #[msg("You are not the recipient of this vault.")]
    InvalidRecipient,

    #[msg("Token transfer failed.")]
    TokenTransferFailed,
}