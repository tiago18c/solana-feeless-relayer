use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
#[error_code]
pub enum SponsorRelayerError {
    #[msg("Invalid tip account")]
    InvalidTipAccount,
    #[msg("Invalid mint")]
    InvalidMint,
    #[msg("Invalid close authority")]
    InvalidCloseAuthority,
    #[msg("Close authority not set")]
    CloseAuthorityNotSet,
    #[msg("Invalid program")]
    InvalidProgram,
    #[msg("Max priority fee exceeded")]
    MaxPriorityFeeExceeded,
}
