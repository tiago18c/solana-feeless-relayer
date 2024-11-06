use anchor_lang::prelude::*;

declare_id!("5Gdnpj8THruSLpvAmP4x9V2YThPT633BiBa9vHvGyXBz");

pub mod error;
pub mod instructions;
pub mod state;

pub use error::*;
pub use instructions::*;
pub use state::*;

#[program]
pub mod sponsor_relayer {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
