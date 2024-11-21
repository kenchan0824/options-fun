use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

use crate::state::*;

pub fn create_market (
    ctx: Context<CreateMarket>,
    premium_rate: u8,
    expiry_days: u8,
    price_tick_size: u8,   
    price_tick_decimals: u8,
    lot_notional_base_amount: u64,
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
    market.price_tick_decimals = price_tick_decimals;
    market.lot_notional_base_amount = lot_notional_base_amount;

    market.lot_premium_base_amount = lot_notional_base_amount / 1000 * premium_rate as u64;
    market.base_decimals = base_mint.decimals;
    market.quote_decimals = quote_mint.decimals;
    market.owner = ctx.accounts.owner.key();

    Ok(())
}

#[derive(Accounts)]
#[instruction(premium_rate: u8, expiry_days: u8)]
pub struct CreateMarket<'info> {
    #[account(
        init, 
        payer = owner, 
        space = 8 + Market::INIT_SPACE,
        seeds = [
            b"market",
            base_mint.key().as_ref(),
            quote_mint.key().as_ref(),
            &[premium_rate],
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
