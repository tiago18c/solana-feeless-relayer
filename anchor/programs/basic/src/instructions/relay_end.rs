use anchor_lang::prelude::*;
use crate::{state::Config, RelayerError};
use anchor_lang::solana_program::sysvar::instructions as tx_instructions;

#[derive(Accounts)]
pub struct RelayEnd<'info> {
    pub signer: Signer<'info>,

    pub config: Account<'info, Config>,

    #[account(address = tx_instructions::ID)]
    pub instructions: UncheckedAccount<'info>,
}

pub fn handler(ctx: Context<RelayEnd>) -> Result<()> {
    Ok(())
}
