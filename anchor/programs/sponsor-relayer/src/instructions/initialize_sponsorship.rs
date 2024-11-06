use anchor_lang::prelude::*;

use crate::Sponsorship;

#[derive(Accounts)]
pub struct InitializeSponsorship<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(init, payer = authority, space = 8 + Sponsorship::INIT_SPACE, seeds = [b"sponsorship"], bump)]
    pub sponsorship: Account<'info, Sponsorship>,
    pub mint: Option<UncheckedAccount<'info>>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_sponsorship(ctx: Context<InitializeSponsorship>, max_priority_fee: u64, allow_permissionless_relayers: bool) -> Result<()> {
    let sponsorship = &mut ctx.accounts.sponsorship;
    sponsorship.authority = *ctx.accounts.authority.key;
    sponsorship.max_priority_fee = max_priority_fee;
    sponsorship.allow_permissionless_relayers = allow_permissionless_relayers;
    sponsorship.bump = ctx.bumps.sponsorship;

    if let Some(mint) = ctx.accounts.mint.as_ref() {
        sponsorship.mint = Some(*mint.key);
    }

    Ok(())
}