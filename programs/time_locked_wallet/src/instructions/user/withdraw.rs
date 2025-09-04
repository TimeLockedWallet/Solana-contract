use anchor_lang::{
    prelude::*, 
    solana_program::{program::invoke_signed, system_instruction}, 
};
use crate::{
    constants::{BANK_VAULT_SEED, VAULT_SEED}, 
    errors::TimeLockedWalletError, 
    states::Vault, 
    utils::{token_transfer_with_signer}
};
use anchor_spl::{
    associated_token::AssociatedToken, 
    token::Token, 
    token_interface::{Mint, TokenAccount}
};

#[derive(Accounts)]
pub struct WithdrawCtx<'info> {
    #[account(
        mut,
        seeds = [VAULT_SEED, owner.key().as_ref()],
        bump,
        // has_one = recipient,
        // close = recipient
    )]
    pub vault: Account<'info, Vault>,

    /// CHECK: SOL vault (if is_sol)
    #[account(
        mut,
        seeds = [BANK_VAULT_SEED, owner.key().as_ref()],
        bump
    )]
    pub bank_vault: UncheckedAccount<'info>,

    /// Token vault ATA (if !is_sol)
    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = bank_vault
    )]
    pub bank_vault_token_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = recipient
    )]
    pub recipient_token_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(mut)]
    pub recipient: Signer<'info>,

    ///CHECK:
    #[account(mut)]
    pub owner : AccountInfo<'info>,
    pub token_mint: InterfaceAccount<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn withdraw(
    ctx: Context<WithdrawCtx>,
) -> Result<()> {
    let bump = ctx.accounts.vault.bump;
    let amount = ctx.accounts.vault.amount;
    let vault = &mut ctx.accounts.vault;
    let owner = &ctx.accounts.owner.key();
    let clock = Clock::get()?;

   if clock.unix_timestamp < vault.unlock_timestamp {
        return err!(TimeLockedWalletError::TimeLockNotExpired);
   }

    require_keys_eq!(ctx.accounts.recipient.key(), vault.recipient, TimeLockedWalletError::InvalidRecipient);

    let signer_seeds: &[&[&[u8]]] = &[&[
        BANK_VAULT_SEED,
        owner.as_ref(),
        &[bump],
    ]];
    if vault.is_sol {
        //let vault_lamports = ctx.accounts.bank_vault.lamports();
        invoke_signed(
            &system_instruction::transfer(
                &ctx.accounts.bank_vault.key(),
                &ctx.accounts.recipient.key(),
                amount,
            ),
            &[
                ctx.accounts.bank_vault.to_account_info(),
                ctx.accounts.recipient.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            signer_seeds,
        )?;
    } else {
        token_transfer_with_signer(
            ctx.accounts.bank_vault_token_ata.to_account_info(), 
            ctx.accounts.bank_vault.to_account_info(), 
            ctx.accounts.recipient_token_ata.to_account_info(), 
            &ctx.accounts.token_program, 
            signer_seeds, 
            amount
        ).map_err(|_|TimeLockedWalletError::TokenTransferFailed)?;
    }
    vault.amount = 0;
    vault.unlock_timestamp = 0;
    Ok(())
}
