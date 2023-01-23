use anchor_lang::prelude::*;

#[account]
pub struct MessageConfig {
    pub index: u32,
    pub payer: Pubkey,
}
