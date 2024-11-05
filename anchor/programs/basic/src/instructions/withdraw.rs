use anchor_lang::prelude::*;
use crate::{state::Config, RelayerError};

#[derive(Accounts)]
pub struct Withdraw<'info> {
    pub signer: Signer<'info>,

    pub config: Account<'info, Config>,
}   

pub fn handler(ctx: Context<Withdraw>) -> Result<()> {
    Ok(())
}
