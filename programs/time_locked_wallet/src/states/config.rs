use anchor_lang::prelude::*;

#[account]
pub struct Config {
    pub authority: Pubkey,
}