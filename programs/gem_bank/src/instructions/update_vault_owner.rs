use anchor_lang::prelude::*;
use gem_common::errors::ErrorCode;

use crate::state::*;

#[derive(Accounts)]
pub struct UpdateVaultOwner<'info> {
    // bank
    pub bank: Box<Account<'info, Bank>>,

    // vault
    // same rationale for not verifying the PDA as in deposit
    #[account(mut, has_one = bank, has_one = owner)]
    pub vault: Box<Account<'info, Vault>>,
    pub owner: Signer<'info>,
}

pub fn handler(ctx: Context<UpdateVaultOwner>, new_owner: Pubkey) -> ProgramResult {
    let bank = &ctx.accounts.bank;
    let vault = &mut ctx.accounts.vault;

    if Bank::read_flags(bank.flags)?.contains(BankFlags::FREEZE_VAULTS) {
        return Err(ErrorCode::VaultAccessSuspended.into());
    }

    // todo is it wise that we're letting them set the owner w/o checking signature?
    //  what if they accidentally set the wrong one? The vault will be frozen forever.
    vault.owner = new_owner;

    msg!("owner updated to: {}", new_owner);
    Ok(())
}
