use anchor_lang::prelude::*;
use crate::{state::Config, RelayerError};

#[derive(Accounts)]
pub struct Refill<'info> {
    pub config: Account<'info, Config>,

    #[account(mut)]
    pub authority: Signer<'info>,

}

pub fn handler(ctx: Context<Refill>, amount: u64) -> Result<()> {
    Ok(())
}
