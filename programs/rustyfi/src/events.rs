use anchor_lang::prelude::*;

#[event]
pub struct MarketInitialized {
    pub market: Pubkey,
    pub authority: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,
    pub fee_bps: u16,
    pub tick_size: i64,
    pub lot_size: u64,
}

#[event]
pub struct SwapExecuted {
    pub market: Pubkey,
    pub user: Pubkey,
    pub amount_in: u64,
    pub amount_out: u64,
    pub is_base_to_quote: bool,
    pub price_impact: u16,        // In basis points
    pub fee_collected: u64,
    pub timestamp: i64,
}

#[event]
pub struct LiquidityAdded {
    pub market: Pubkey,
    pub user: Pubkey,
    pub base_amount: u64,
    pub quote_amount: u64,
    pub lp_tokens_minted: u64,
    pub total_lp_supply: u64,
    pub timestamp: i64,
}

#[event]
pub struct LiquidityRemoved {
    pub market: Pubkey,
    pub user: Pubkey,
    pub base_amount: u64,
    pub quote_amount: u64,
    pub lp_tokens_burned: u64,
    pub total_lp_supply: u64,
    pub timestamp: i64,
}

#[event]
pub struct PriceOracleUpdated {
    pub market: Pubkey,
    pub old_price: u64,
    pub new_price: u64,
    pub confidence: u64,
    pub timestamp: i64,
}

#[event]
pub struct PositionLiquidated {
    pub market: Pubkey,
    pub borrower: Pubkey,
    pub liquidator: Pubkey,
    pub collateral_seized: u64,
    pub debt_repaid: u64,
    pub liquidation_bonus: u64,
    pub timestamp: i64,
}

#[event]
pub struct InterestAccrued {
    pub market: Pubkey,
    pub total_borrows: u64,
    pub total_reserves: u64,
    pub borrow_rate: u16,
    pub supply_rate: u16,
    pub timestamp: i64,
}

#[event]
pub struct EmergencyPaused {
    pub market: Pubkey,
    pub reason: String,
    pub paused_by: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct FlashLoanExecuted {
    pub market: Pubkey,
    pub borrower: Pubkey,
    pub amount: u64,
    pub fee: u64,
    pub timestamp: i64,
}

#[event]
pub struct GovernanceProposalCreated {
    pub proposal_id: u64,
    pub proposer: Pubkey,
    pub voting_start: i64,
    pub voting_end: i64,
    pub description: String,
}
