use anchor_lang::prelude::*;

pub mod state;
pub mod events;
pub mod ix;

// Re-export types so the #[program] fn signature can see them
pub use ix::{InitializeMarket, InitializeParams};

declare_id!("62rXFy9Cp9V773CeajjBHims3mDveDynF4UDHJqDY1F5");

#[program]
pub mod rustyfi {
    use super::*;

    pub fn initialize_market(
        ctx: Context<InitializeMarket>,
        params: InitializeParams,
    ) -> Result<()> {
        ix::initialize_market::initialize_market(ctx, params)
    }
}
