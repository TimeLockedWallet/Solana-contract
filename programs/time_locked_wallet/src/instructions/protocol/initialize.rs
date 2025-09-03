use anchor_lang::prelude::*;

use crate::{
    states::config::Config,
    constants::CONFIG_SEED
};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + std::mem::size_of::<Config>(),
        seeds = [CONFIG_SEED],              
        bump
    )]
    pub config: Account<'info, Config>, 

    #[account(mut)]
    pub authority: Signer<'info>,               
    pub system_program: Program<'info, System>,
}
pub fn initialize(
    ctx: Context<Initialize>, 
) -> Result<()> {

    let config = &mut ctx.accounts.config;
    config.authority = ctx.accounts.authority.key();
    msg!("Vault initialized",);
    Ok(())
}