use anchor_lang::prelude::*;
use crate::state::Config;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct RegisterArgs {
    pub relayer_wallet: Pubkey,

}

#[derive(Accounts)]
pub struct Register<'info> {
    #[account(
        init,
        payer = authority,
        space = 8+ Config::INIT_SPACE,
        seeds = [Config::SEED_PREFIX],
        bump
    )]
    pub config: Account<'info, Config>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Register>, args: RegisterArgs) -> Result<()> {
    let config = &mut ctx.accounts.config;
    let bump = ctx.bumps.config;

    config.set_inner(Config::new(
        ctx.accounts.authority.key(),
        bump
    ));

    Ok(())
}
