use anchor_lang::prelude::*;

#[error_code]
pub enum RelayerError {
    #[msg("Unauthorized")]
    Unauthorized,

    #[msg("Invalid begin index")]
    InvalidBeginIndex,
}
