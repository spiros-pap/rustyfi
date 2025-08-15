use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::events::MarketInitialized;
use crate::state::{Market, MarketBumps};

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct InitializeParams {
    pub fee_bps: u16,   // 0..=10000
    pub tick_size: i64,
    pub lot_size: u64,
}

#[derive(Accounts)]
#[instruction(params: InitializeParams)]
pub struct InitializeMarket<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub authority: UncheckedAccount<'info>,

    #[account(
        init,
        payer = payer,
        space = 8 + Market::SIZE,
        seeds = [b"market", authority.key().as_ref(), base_mint.key().as_ref(), quote_mint.key().as_ref()],
        bump
    )]
    pub market: Account<'info, Market>,

    pub base_mint: Account<'info, Mint>,
    pub quote_mint: Account<'info, Mint>,

    #[account(
        init,
        payer = payer,
        token::mint = base_mint,
        token::authority = market,
        seeds = [b"base_vault", market.key().as_ref()],
        bump
    )]
    pub base_vault: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = payer,
        token::mint = quote_mint,
        token::authority = market,
        seeds = [b"quote_vault", market.key().as_ref()],
        bump
    )]
    pub quote_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn initialize_market(ctx: Context<InitializeMarket>, params: InitializeParams) -> Result<()> {
    require!(params.fee_bps <= 10_000, ErrorCode::InvalidFeeBps);

    let bumps = MarketBumps {
        market: ctx.bumps.market,
        base_vault: ctx.bumps.base_vault,
        quote_vault: ctx.bumps.quote_vault,
    };

    let m = &mut ctx.accounts.market;
    m.authority   = ctx.accounts.authority.key();
    m.base_mint   = ctx.accounts.base_mint.key();
    m.quote_mint  = ctx.accounts.quote_mint.key();
    m.base_vault  = ctx.accounts.base_vault.key();
    m.quote_vault = ctx.accounts.quote_vault.key();
    m.fee_bps     = params.fee_bps;
    m.tick_size   = params.tick_size;
    m.lot_size    = params.lot_size;
    m.bumps       = bumps;
    m.padding     = [0u8; 64];

    emit!(MarketInitialized {
        market: m.key(),
        authority: m.authority,
        base_mint: m.base_mint,
        quote_mint: m.quote_mint,
        base_vault: m.base_vault,
        quote_vault: m.quote_vault,
        fee_bps: m.fee_bps,
        tick_size: m.tick_size,
        lot_size: m.lot_size,
    });

    Ok(())
}

#[error_code]
pub enum ErrorCode {
    #[msg("Fee basis points must be <= 10000")]
    InvalidFeeBps,
}
