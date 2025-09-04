#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod states;
pub mod utils;

use instructions::*;
declare_id!("fi79pH1tGywgPZWmoStRoYgNrUFYvoJZAM2wqDb8yjK");

#[program]
pub mod time_locked_wallet {
    use super::*;
    //protocol instructions
    pub fn initialize(
        ctx: Context<Initialize>,
    ) -> Result<()> {
        instructions::protocol::initialize::initialize(
            ctx, 
        )
    }
    //user instruction
    pub fn initialize_lock(
        ctx: Context<InitializeLock>,
        unlock_timestamp : i64,
        recipient : Pubkey,
        amount : u64,
        is_sol : bool,
    ) -> Result<()> {
        instructions::user::initialize_lock::initialize_lock(
            ctx, 
            unlock_timestamp,
            recipient,
            amount,
            is_sol
        )
    }

     pub fn withdraw(
        ctx: Context<WithdrawCtx>,
    ) -> Result<()> {
        instructions::user::withdraw::withdraw(
            ctx, 
        )
    }
}

