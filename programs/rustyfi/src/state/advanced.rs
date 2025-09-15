use anchor_lang::prelude::*;

/// Liquidity Pool state for tracking AMM statistics and metrics
#[account]
pub struct LiquidityPool {
    pub market: Pubkey,
    pub total_volume: u64,
    pub swap_count: u64,
    pub fees_collected_base: u64,
    pub fees_collected_quote: u64,
    pub created_at: i64,
    pub last_updated: i64,
    pub bump: u8,
    pub padding: [u8; 128], // Future extensibility
}

impl LiquidityPool {
    pub const SIZE: usize = 
        32 +    // market
        8 +     // total_volume
        8 +     // swap_count
        8 +     // fees_collected_base
        8 +     // fees_collected_quote
        8 +     // created_at
        8 +     // last_updated
        1 +     // bump
        128;    // padding
}

/// User position tracking for advanced features
#[account]
pub struct UserPosition {
    pub owner: Pubkey,
    pub market: Pubkey,
    pub base_deposited: u64,
    pub quote_deposited: u64,
    pub lp_tokens: u64,
    pub last_interaction: i64,
    pub total_fees_earned: u64,
    pub bump: u8,
    pub padding: [u8; 64],
}

impl UserPosition {
    pub const SIZE: usize = 
        32 +    // owner
        32 +    // market
        8 +     // base_deposited
        8 +     // quote_deposited
        8 +     // lp_tokens
        8 +     // last_interaction
        8 +     // total_fees_earned
        1 +     // bump
        64;     // padding
}

/// Oracle price data with staleness checks
#[account]
pub struct PriceOracle {
    pub market: Pubkey,
    pub pyth_price_account: Pubkey,
    pub max_staleness: i64,      // Maximum age in seconds
    pub confidence_threshold: u64, // Maximum acceptable confidence interval
    pub last_update: i64,
    pub emergency_mode: bool,     // Fallback when oracle fails
    pub twap_period: u32,        // Time-weighted average price period
    pub bump: u8,
    pub padding: [u8; 32],
}

impl PriceOracle {
    pub const SIZE: usize = 
        32 +    // market
        32 +    // pyth_price_account
        8 +     // max_staleness
        8 +     // confidence_threshold
        8 +     // last_update
        1 +     // emergency_mode
        4 +     // twap_period
        1 +     // bump
        32;     // padding
}

/// Lending pool state for borrowing/lending functionality
#[account]
pub struct LendingPool {
    pub market: Pubkey,
    pub total_deposits: u64,
    pub total_borrows: u64,
    pub utilization_rate: u16,    // In basis points
    pub interest_rate: u16,       // Annual rate in basis points
    pub reserve_factor: u16,      // Protocol fee in basis points
    pub last_update: i64,
    pub liquidation_threshold: u16, // Collateral ratio for liquidation
    pub bump: u8,
    pub padding: [u8; 64],
}

impl LendingPool {
    pub const SIZE: usize = 
        32 +    // market
        8 +     // total_deposits
        8 +     // total_borrows
        2 +     // utilization_rate
        2 +     // interest_rate
        2 +     // reserve_factor
        8 +     // last_update
        2 +     // liquidation_threshold
        1 +     // bump
        64;     // padding

    /// Calculate utilization rate: borrows / deposits
    pub fn calculate_utilization(&self) -> u16 {
        if self.total_deposits == 0 {
            return 0;
        }
        
        ((self.total_borrows * 10000) / self.total_deposits) as u16
    }

    /// Calculate interest rate based on utilization
    pub fn calculate_interest_rate(&self) -> u16 {
        let utilization = self.calculate_utilization();
        
        // Simple linear model: base_rate + utilization * slope
        let base_rate = 200;  // 2% base rate
        let slope = 1000;     // 10% at 100% utilization
        
        base_rate + (utilization * slope / 10000)
    }
}