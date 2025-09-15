use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::state::{Market, LiquidityPool};
use crate::events::SwapExecuted;
use crate::error::RustyfiError;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct SwapParams {
    pub amount_in: u64,
    pub minimum_amount_out: u64,  // Slippage protection
    pub is_base_to_quote: bool,   // Direction of swap
}

#[derive(Accounts)]
#[instruction(params: SwapParams)]
pub struct Swap<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"market", market.authority.as_ref(), market.base_mint.as_ref(), market.quote_mint.as_ref()],
        bump = market.bumps.market,
    )]
    pub market: Account<'info, Market>,

    #[account(
        mut,
        seeds = [b"pool", market.key().as_ref()],
        bump = pool.bump,
    )]
    pub pool: Account<'info, LiquidityPool>,

    // User's token accounts
    #[account(
        mut,
        constraint = user_base_account.owner == user.key(),
        constraint = user_base_account.mint == market.base_mint,
    )]
    pub user_base_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = user_quote_account.owner == user.key(),
        constraint = user_quote_account.mint == market.quote_mint,
    )]
    pub user_quote_account: Account<'info, TokenAccount>,

    // Market vaults
    #[account(
        mut,
        seeds = [b"base_vault", market.key().as_ref()],
        bump = market.bumps.base_vault,
    )]
    pub base_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"quote_vault", market.key().as_ref()],
        bump = market.bumps.quote_vault,
    )]
    pub quote_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn swap(ctx: Context<Swap>, params: SwapParams) -> Result<()> {
    let market = &ctx.accounts.market;
    let pool = &mut ctx.accounts.pool;

    // SECURITY: Comprehensive input validation
    require!(params.amount_in > 0, RustyfiError::InvalidAmount);
    require!(params.minimum_amount_out > 0, RustyfiError::InvalidMinimumAmount);
    
    // Prevent dust attacks and ensure meaningful swap amounts
    require!(params.amount_in >= 1000, RustyfiError::SwapAmountTooSmall);
    
    // Validate user has sufficient balance before proceeding
    let user_balance = if params.is_base_to_quote {
        ctx.accounts.user_base_account.amount
    } else {
        ctx.accounts.user_quote_account.amount
    };
    require!(user_balance >= params.amount_in, RustyfiError::InsufficientBalance);

    // Get current reserves from vaults
    let base_reserve = ctx.accounts.base_vault.amount;
    let quote_reserve = ctx.accounts.quote_vault.amount;

    // Ensure liquidity exists
    require!(base_reserve > 0 && quote_reserve > 0, RustyfiError::InsufficientLiquidity);

    // Calculate swap using constant product formula (x * y = k)
    let (amount_out, new_base_reserve, new_quote_reserve) = if params.is_base_to_quote {
        // Selling base for quote
        // SECURITY: Add full amount to reserves first, then deduct fees
        let new_base_reserve = base_reserve
            .checked_add(params.amount_in)
            .ok_or(RustyfiError::MathOverflow)?;

        // Apply fee to effective amount for calculation
        let amount_in_with_fee = params.amount_in
            .checked_mul(10000 - market.fee_bps as u64)
            .ok_or(RustyfiError::MathOverflow)?
            .checked_div(10000)
            .ok_or(RustyfiError::MathOverflow)?;

        // Calculate amount out: dy = y * dx / (x + dx)
        let numerator = quote_reserve
            .checked_mul(amount_in_with_fee)
            .ok_or(RustyfiError::MathOverflow)?;
        
        let denominator = base_reserve
            .checked_add(amount_in_with_fee)
            .ok_or(RustyfiError::MathOverflow)?;

        let amount_out = numerator
            .checked_div(denominator)
            .ok_or(RustyfiError::MathOverflow)?;

        let new_quote_reserve = quote_reserve
            .checked_sub(amount_out)
            .ok_or(RustyfiError::InsufficientLiquidity)?;

        (amount_out, new_base_reserve, new_quote_reserve)
    } else {
        // Selling quote for base
        // SECURITY: Add full amount to reserves first, then deduct fees
        let new_quote_reserve = quote_reserve
            .checked_add(params.amount_in)
            .ok_or(RustyfiError::MathOverflow)?;

        let amount_in_with_fee = params.amount_in
            .checked_mul(10000 - market.fee_bps as u64)
            .ok_or(RustyfiError::MathOverflow)?
            .checked_div(10000)
            .ok_or(RustyfiError::MathOverflow)?;

        let numerator = base_reserve
            .checked_mul(amount_in_with_fee)
            .ok_or(RustyfiError::MathOverflow)?;

        let denominator = quote_reserve
            .checked_add(amount_in_with_fee)
            .ok_or(RustyfiError::MathOverflow)?;

        let amount_out = numerator
            .checked_div(denominator)
            .ok_or(RustyfiError::MathOverflow)?;

        let new_base_reserve = base_reserve
            .checked_sub(amount_out)
            .ok_or(RustyfiError::InsufficientLiquidity)?;

        (amount_out, new_base_reserve, new_quote_reserve)
    };

    // Slippage protection
    require!(amount_out >= params.minimum_amount_out, RustyfiError::SlippageExceeded);

    // Calculate price impact for monitoring
    let price_impact = calculate_price_impact(
        params.amount_in,
        amount_out,
        base_reserve,
        quote_reserve,
        params.is_base_to_quote,
    )?;

    // Warn for high price impact (>5%)
    if price_impact > 500 { // 5% in basis points
        msg!("Warning: High price impact detected: {}bps", price_impact);
    }

    // Execute the swap transfers
    let market_seeds = &[
        b"market",
        market.authority.as_ref(),
        market.base_mint.as_ref(),
        market.quote_mint.as_ref(),
        &[market.bumps.market],
    ];
    let signer_seeds = &[&market_seeds[..]];

    if params.is_base_to_quote {
        // Transfer base tokens from user to vault
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.user_base_account.to_account_info(),
                    to: ctx.accounts.base_vault.to_account_info(),
                    authority: ctx.accounts.user.to_account_info(),
                },
            ),
            params.amount_in,
        )?;

        // Transfer quote tokens from vault to user
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.quote_vault.to_account_info(),
                    to: ctx.accounts.user_quote_account.to_account_info(),
                    authority: ctx.accounts.market.to_account_info(),
                },
                signer_seeds,
            ),
            amount_out,
        )?;
    } else {
        // Transfer quote tokens from user to vault
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.user_quote_account.to_account_info(),
                    to: ctx.accounts.quote_vault.to_account_info(),
                    authority: ctx.accounts.user.to_account_info(),
                },
            ),
            params.amount_in,
        )?;

        // Transfer base tokens from vault to user
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.base_vault.to_account_info(),
                    to: ctx.accounts.user_base_account.to_account_info(),
                    authority: ctx.accounts.market.to_account_info(),
                },
                signer_seeds,
            ),
            amount_out,
        )?;
    }

    // Update pool statistics
    pool.total_volume = pool.total_volume
        .checked_add(params.amount_in)
        .ok_or(RustyfiError::MathOverflow)?;

    pool.swap_count = pool.swap_count
        .checked_add(1)
        .ok_or(RustyfiError::MathOverflow)?;

    // Emit swap event with timestamp for monitoring
    emit!(SwapExecuted {
        market: market.key(),
        user: ctx.accounts.user.key(),
        amount_in: params.amount_in,
        amount_out,
        is_base_to_quote: params.is_base_to_quote,
        price_impact,
        fee_collected: params.amount_in - (params.amount_in * (10000 - market.fee_bps as u64) / 10000),
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}

/// Calculate price impact in basis points
fn calculate_price_impact(
    amount_in: u64,
    amount_out: u64,
    reserve_in: u64,
    reserve_out: u64,
    is_base_to_quote: bool,
) -> Result<u16> {
    // Price before swap
    let price_before = if is_base_to_quote {
        reserve_out
            .checked_mul(10000)
            .ok_or(RustyfiError::MathOverflow)?
            .checked_div(reserve_in)
            .ok_or(RustyfiError::MathOverflow)?
    } else {
        reserve_in
            .checked_mul(10000)
            .ok_or(RustyfiError::MathOverflow)?
            .checked_div(reserve_out)
            .ok_or(RustyfiError::MathOverflow)?
    };

    // Effective price of this trade
    let trade_price = amount_out
        .checked_mul(10000)
        .ok_or(RustyfiError::MathOverflow)?
        .checked_div(amount_in)
        .ok_or(RustyfiError::MathOverflow)?;

    // Calculate impact as percentage difference
    let price_diff = if price_before > trade_price {
        price_before - trade_price
    } else {
        trade_price - price_before
    };

    let impact = price_diff
        .checked_mul(10000)
        .ok_or(RustyfiError::MathOverflow)?
        .checked_div(price_before)
        .ok_or(RustyfiError::MathOverflow)?;

    Ok(impact as u16)
}