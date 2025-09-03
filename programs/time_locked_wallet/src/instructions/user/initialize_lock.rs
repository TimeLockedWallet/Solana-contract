use anchor_lang::{prelude::*, system_program};
use crate::{
    constants::{BANK_VAULT_SEED, VAULT_SEED}, 
    errors::TimeLockedWalletError, 
    states::Vault, 
    utils::{sol_transfer_from_user, token_transfer_user}
};
use anchor_spl::{
    associated_token::AssociatedToken, 
    token::Token, 
    token_interface::{Mint, TokenAccount}
};

#[derive(Accounts)]
pub struct InitializeLock<'info> {

    #[account(
        init_if_needed,
        payer = user,
        space = 8 + std::mem::size_of::<Vault>(),
        seeds = [VAULT_SEED, user.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,

    ///CHECK:
    #[account(
        init_if_needed, 
        payer = user,
        space = 0,
        seeds = [BANK_VAULT_SEED, user.key().as_ref()],
        bump,
        owner = system_program::ID,
    )]
    pub bank_vault: UncheckedAccount<'info>,

    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = user,
    )]
    pub user_token_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = bank_vault,
    )]
    pub bank_vault_token_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub token_mint: InterfaceAccount<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_lock (
    ctx : Context<InitializeLock>, 
    unlock_timestamp : i64,
    recipient : Pubkey,
    amount : u64,
    is_sol : bool,
) -> Result<()> {

    let vault = &mut ctx.accounts.vault;
    
    if vault.unlock_timestamp != 0 {
        return err!(TimeLockedWalletError::TimeLockNotExpired);
    }

    vault.amount = amount;
    vault.unlock_timestamp = unlock_timestamp;
    vault.recipient = recipient;
    vault.is_sol = is_sol;
    vault.bump = ctx.bumps.bank_vault;

    if is_sol {
        sol_transfer_from_user(
            &ctx.accounts.user,
            ctx.accounts.bank_vault.to_account_info(),
            &ctx.accounts.system_program,
            amount
        ).map_err(|_|TimeLockedWalletError::TokenTransferFailed)?;

    } else {
        token_transfer_user(
            ctx.accounts.user_token_ata.to_account_info(),
            &ctx.accounts.user,
            ctx.accounts.bank_vault_token_ata.to_account_info(),
            &ctx.accounts.token_program,
            amount,
        ).map_err(|_|TimeLockedWalletError::TokenTransferFailed)?;
    }

    msg!("Lock initialized successfully. Unlock timestamp: {}", unlock_timestamp);
    Ok(()) 
}