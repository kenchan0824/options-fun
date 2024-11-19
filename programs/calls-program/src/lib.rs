use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

mod state;
use state::{Market};

declare_id!("d67T6FJT4Hnhp8FURL4V71XB9fkSzXmp7wUN4XrCxbC");

#[program]
pub mod calls_program {
    use super::*;

    pub fn create_market(
        ctx: Context<CreateMarket>,
        premium_rate: u16,
        expiry_days: u8,
        price_tick_size: u16,   
        price_decimals: u8,
        lot_notional_base_amount: u64,
        lot_premium_base_amount: u64, 
    ) -> Result<()> {

        let market = &mut ctx.accounts.market;
        let base_mint = &ctx.accounts.base_mint;
        let quote_mint = &ctx.accounts.quote_mint;


        market.base_mint = ctx.accounts.base_mint.key();
        market.quote_mint = ctx.accounts.quote_mint.key();
        market.premium_rate = premium_rate;
        market.expiry_days = expiry_days;
        market.bump = ctx.bumps.market;

        market.price_tick_size = price_tick_size;
        market.price_decimals = price_decimals;
        market.lot_notional_base_amount = lot_notional_base_amount;
        market.lot_premium_base_amount = lot_premium_base_amount;
        market.base_decimals = base_mint.decimals;
        market.quote_decimals = quote_mint.decimals;

        market.owner = ctx.accounts.owner.key();

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(premium_rate: u16, expiry_days: u8)]
pub struct CreateMarket<'info> {
    #[account(
        init, 
        payer = owner, 
        space = 8 + Market::INIT_SPACE,
        seeds = [
            b"market",
            base_mint.key().as_ref(),
            quote_mint.key().as_ref(),
            premium_rate.to_le_bytes().as_ref(),
            &[expiry_days],
        ],
        bump,
    )]
    pub market: Account<'info, Market>,

    pub base_mint: Account<'info, Mint>,
    pub quote_mint: Account<'info, Mint>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}
