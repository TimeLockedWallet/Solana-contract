use anchor_lang::prelude::*;

#[account]
pub struct Vault {
    pub amount : u64,
    pub unlock_timestamp: i64,
    pub recipient: Pubkey,
    pub is_sol : bool,
    pub bump : u8
}