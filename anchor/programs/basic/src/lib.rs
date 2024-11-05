use anchor_lang::prelude::*;

declare_id!("6z68wfurCMYkZG51s1Et9BJEd9nJGUusjHXNt4dGbNNF");

pub mod error;
pub mod instructions;
pub mod state;

pub use error::*;
pub use instructions::*;
pub use state::*;

#[program]
pub mod basic {
    use super::*;

    pub fn register(ctx: Context<Register>, args: RegisterArgs) -> Result<()> {
        instructions::register::handler(ctx, args)
    }

    pub fn refill(ctx: Context<Refill>, amount: u64) -> Result<()> {
        instructions::refill::handler(ctx, amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        instructions::withdraw::handler(ctx)
    }

    pub fn relay_begin(ctx: Context<RelayBegin>) -> Result<()> {
        instructions::relay_begin::handler(ctx)
    }

    pub fn relay_end(ctx: Context<RelayEnd>) -> Result<()> {
        instructions::relay_end::handler(ctx)
    }   
}
