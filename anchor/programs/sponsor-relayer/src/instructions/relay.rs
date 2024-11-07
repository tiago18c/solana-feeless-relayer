use anchor_lang::{prelude::*, solana_program::rent};
use anchor_lang::solana_program::sysvar::instructions as tx_instructions;
use anchor_spl::associated_token;

use crate::{Relayer, SponsorRelayerError, Sponsorship};

const TRANSFER_CHECKED_DISCRIMINATOR: u8 = 12;
const INITIALIZE_ACCOUNT_2_DISCRIMINATOR: u8 = 16;
const INITIALIZE_ACCOUNT_3_DISCRIMINATOR: u8 = 18;
const SET_AUTHORITY_DISCRIMINATOR: u8 = 6;

const ATA_CREATE_DISCRIMINATOR: u8 = 0;
const ATA_CREATE_IDEMPOTENT_DISCRIMINATOR: u8 = 1;

const CLOSE_AUTHORITY_TYPE: u8 = 3;

pub mod compute_budget {
    use anchor_lang::declare_id;

    pub const COMPUTE_LIMIT_DISCRIMINATOR: u8 = 2;
    pub const COMPUTE_PRICE_DISCRIMINATOR: u8 = 3;
    declare_id!("ComputeBudget111111111111111111111111111111");
}

#[derive(Accounts)]
pub struct Relay<'info> {
    #[account(mut)]
    pub relayer_wallet: Signer<'info>,
    #[account(has_one = relayer_wallet, has_one = sponsorship)]
    pub relayer: Account<'info, Relayer>,
    #[account(mut)]
    pub sponsorship: Account<'info, Sponsorship>,
    /// CHECK: instructions sysvar
    #[account(address = tx_instructions::ID)]
    pub instructions: UncheckedAccount<'info>,
}

pub fn handler(ctx: Context<Relay>) -> Result<()> {
    let mut index = 0;
    let mut signers = vec![];
    let mut compute_limit = 0;
    let mut compute_price = 0;
    let mut init_accounts = vec![];
    let mut rent : u64 = 0;

    while let Ok(instr) = tx_instructions::load_instruction_at_checked(index, &ctx.accounts.instructions) {

        instr.accounts.iter().for_each(|account| if account.is_signer {
            if !signers.contains(&account.pubkey) {
                signers.push(account.pubkey);
            }
        });

        if instr.program_id == compute_budget::id() {
            if instr.data[0] == compute_budget::COMPUTE_LIMIT_DISCRIMINATOR {
                compute_limit = u64::from_le_bytes(instr.data[1..].try_into().unwrap());
            } else if instr.data[0] == compute_budget::COMPUTE_PRICE_DISCRIMINATOR {
                compute_price = u64::from_le_bytes(instr.data[1..].try_into().unwrap());
            }
        } else if instr.program_id == crate::id() {
            //curr prog
        } else if instr.program_id == anchor_spl::token::ID || instr.program_id == anchor_spl::token_2022::ID {
            //spl token
            // check if it's a transfer
            if instr.data[0] == TRANSFER_CHECKED_DISCRIMINATOR {
                if let Some(mint) = ctx.accounts.sponsorship.mint {
                    require!(instr.accounts[1].pubkey == mint, SponsorRelayerError::InvalidMint);
                }
            } else if instr.data[0] == INITIALIZE_ACCOUNT_2_DISCRIMINATOR || instr.data[0] == INITIALIZE_ACCOUNT_3_DISCRIMINATOR {
                init_accounts.push(instr.accounts[0].pubkey);
            } else if instr.data[0] == SET_AUTHORITY_DISCRIMINATOR && instr.data[1] == CLOSE_AUTHORITY_TYPE {
                require!(instr.data[3..] == ctx.accounts.relayer_wallet.key().to_bytes(), SponsorRelayerError::InvalidCloseAuthority);
                if let Some(idx) = init_accounts.iter().position(|&x| x == instr.accounts[0].pubkey) {
                    let acc = init_accounts.remove(idx);

                    rent += ctx.remaining_accounts.iter().find(|x| x.key == &acc).unwrap().lamports();
                } else {
                    return Err(SponsorRelayerError::CloseAuthorityNotSet.into());
                }
            }
        } else if instr.program_id == associated_token::ID {
            // more work
            if instr.data.len() == 0 || instr.data[0] == ATA_CREATE_DISCRIMINATOR || instr.data[0] == ATA_CREATE_IDEMPOTENT_DISCRIMINATOR {
                init_accounts.push(instr.accounts[1].pubkey);
                //mint at idx 3
            }
        } else {
            return Err(SponsorRelayerError::InvalidProgram.into());
        }
        
        index += 1;
    }

    if init_accounts.len() > 0 {
        return Err(SponsorRelayerError::CloseAuthorityNotSet.into());
    }

    let price = compute_price * (if compute_limit == 0 { 200000 * (index as u64 - 1) } else { compute_limit });

    if price > ctx.accounts.sponsorship.max_priority_fee {
        return Err(SponsorRelayerError::MaxPriorityFeeExceeded.into());
    }

    let price = price / 1_000_000 + signers.len() as u64 * 5_000 + rent;

    msg!("price: {}", price);

    ctx.accounts.relayer_wallet.add_lamports(price)?;
    ctx.accounts.sponsorship.sub_lamports(price)?;
    Ok(())
}
