pub mod state;
pub mod markets;

use anchor_lang::prelude::*;
use markets::*;

declare_id!("d67T6FJT4Hnhp8FURL4V71XB9fkSzXmp7wUN4XrCxbC");

#[program]
pub mod calls_program {
    use super::*;

    pub fn create_market(
        ctx: Context<CreateMarket>,
        premium_rate: u8,
        expiry_days: u8,
        price_tick_size: u8,   
        price_tick_decimals: u8,
        lot_notional_base_amount: u64,
    ) -> Result<()> {
        markets::create_market(ctx, premium_rate, expiry_days, price_tick_size, price_tick_decimals, lot_notional_base_amount)
    }

}
