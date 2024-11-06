use anchor_lang::prelude::*;

use crate::{Relayer, Sponsorship};

#[derive(Accounts)]
pub struct AuthorizeRelayer<'info> {
    #[account()]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub relayer: Account<'info, Relayer>,
    #[account(has_one = authority)]
    pub sponsorship: Account<'info, Sponsorship>,
}

pub fn authorize_relayer(ctx: Context<AuthorizeRelayer>) -> Result<()> {
    let relayer = &mut ctx.accounts.relayer;
    relayer.authorized = true;
    Ok(())
}