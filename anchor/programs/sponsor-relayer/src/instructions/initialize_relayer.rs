use anchor_lang::prelude::*;

use crate::{Relayer, Sponsorship};

#[derive(Accounts)]
pub struct InitializeRelayer<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(init, payer = authority, space = 8 + Relayer::INIT_SPACE, seeds = [b"relayer"], bump)]
    pub relayer: Account<'info, Relayer>,
    pub sponsorship: Account<'info, Sponsorship>,
    /// CHECK: arbitrary relayer wallet
    pub relayer_wallet: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_relayer(ctx: Context<InitializeRelayer>, sponsor_cap: u64) -> Result<()> {
    let relayer = &mut ctx.accounts.relayer;
    relayer.authority = ctx.accounts.authority.key();
    relayer.sponsorship = ctx.accounts.sponsorship.key();
    relayer.sponsor_cap = sponsor_cap;
    relayer.authorized = ctx.accounts.sponsorship.allow_permissionless_relayers;
    relayer.bump = ctx.bumps.relayer;
    Ok(())
}
