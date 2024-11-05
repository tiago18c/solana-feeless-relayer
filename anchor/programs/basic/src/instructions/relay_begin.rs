use anchor_lang::prelude::*;
use crate::{state::Config, RelayerError};

use anchor_lang::solana_program::sysvar::instructions as tx_instructions;

#[derive(Accounts)]
pub struct RelayBegin<'info> {
    pub signer: Signer<'info>,

    pub config: Account<'info, Config>,

    #[account(address = tx_instructions::ID)]
    pub instructions: UncheckedAccount<'info>,
}

pub fn handler(ctx: Context<RelayBegin>) -> Result<()> {
    let index = tx_instructions::load_current_index_checked(&ctx.accounts.instructions)?;

    require!(index == 0, RelayerError::InvalidBeginIndex);

    // do the rest of introspection
    Ok(())
}
