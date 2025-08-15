use anchor_lang::prelude::*;

#[account]
pub struct Market {
    pub authority: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,
    pub fee_bps: u16,
    pub tick_size: i64,
    pub lot_size: u64,
    pub bumps: MarketBumps,
    pub padding: [u8; 64],
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct MarketBumps {
    pub market: u8,
    pub base_vault: u8,
    pub quote_vault: u8,
}

impl MarketBumps {
    pub const SIZE: usize = 3;
}

impl Market {
    pub const SIZE: usize =
        32*5 +   // five Pubkeys
        2   +    // fee_bps
        8   +    // tick_size
        8   +    // lot_size
        MarketBumps::SIZE +
        64;      // padding
}
