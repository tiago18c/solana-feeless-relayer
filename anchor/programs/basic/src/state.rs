use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub authority: Pubkey,
    pub bump: u8,
}

impl Config {
    pub const SEED_PREFIX: &'static [u8] = b"config";
    pub const SPACE: usize = 8 + // discriminator
        32 + // authority pubkey
        1;  // bump

    pub fn new(authority: Pubkey, bump: u8) -> Self {
        Config {
            authority,
            bump,
        }
    }
}
