use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Market {
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub premium_rate: u8,
    pub expiry_days: u8,
    pub bump: u8,

    pub owner: Pubkey,
    pub lot_notional_base_amount: u64,
    pub lot_premium_base_amount: u64,
    pub base_decimals: u8,
    pub quote_decimals: u8,
}