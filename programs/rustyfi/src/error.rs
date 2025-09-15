use anchor_lang::prelude::*;

/// Comprehensive error codes for professional-grade DeFi protocol
#[error_code]
pub enum RustyfiError {
    // Market errors (1000-1099)
    #[msg("Market is not initialized")]
    MarketNotInitialized = 1000,

    #[msg("Market is paused")]
    MarketPaused = 1001,

    #[msg("Invalid market authority")]
    InvalidMarketAuthority = 1002,

    #[msg("Market already exists")]
    MarketAlreadyExists = 1003,

    // Token and account errors (1100-1199)
    #[msg("Invalid mint provided")]
    InvalidMint = 1100,

    #[msg("Invalid token account")]
    InvalidTokenAccount = 1101,

    #[msg("Token account owner mismatch")]
    TokenAccountOwnerMismatch = 1102,

    #[msg("Insufficient token balance")]
    InsufficientBalance = 1103,

    #[msg("Token transfer failed")]
    TokenTransferFailed = 1104,

    // Swap and trading errors (1200-1299)
    #[msg("Invalid swap amount - must be greater than 0")]
    InvalidAmount = 1200,

    #[msg("Invalid minimum amount out")]
    InvalidMinimumAmount = 1201,

    #[msg("Insufficient liquidity in pool")]
    InsufficientLiquidity = 1202,

    #[msg("Slippage tolerance exceeded")]
    SlippageExceeded = 1203,

    #[msg("Swap amount too small")]
    SwapAmountTooSmall = 1204,

    #[msg("Swap amount too large - exceeds pool capacity")]
    SwapAmountTooLarge = 1205,

    #[msg("Price impact too high")]
    PriceImpactTooHigh = 1206,

    // Mathematical errors (1300-1399)
    #[msg("Mathematical overflow occurred")]
    MathOverflow = 1300,

    #[msg("Mathematical underflow occurred")]
    MathUnderflow = 1301,

    #[msg("Division by zero")]
    DivisionByZero = 1302,

    #[msg("Invalid calculation result")]
    InvalidCalculation = 1303,

    // Fee and configuration errors (1400-1499)
    #[msg("Fee basis points must be <= 10000")]
    InvalidFeeBps = 1400,

    #[msg("Fee collection failed")]
    FeeCollectionFailed = 1401,

    #[msg("Invalid fee recipient")]
    InvalidFeeRecipient = 1402,

    // Liquidity errors (1500-1599)
    #[msg("Invalid liquidity amount")]
    InvalidLiquidityAmount = 1500,

    #[msg("Insufficient LP tokens")]
    InsufficientLpTokens = 1501,

    #[msg("Liquidity deposit failed")]
    LiquidityDepositFailed = 1502,

    #[msg("Liquidity withdrawal failed")]
    LiquidityWithdrawalFailed = 1503,

    #[msg("LP token mint failed")]
    LpTokenMintFailed = 1504,

    // Oracle and price errors (1600-1699)
    #[msg("Oracle price is stale")]
    StalePriceData = 1600,

    #[msg("Oracle confidence too low")]
    LowOracleConfidence = 1601,

    #[msg("Price feed not available")]
    PriceFeedUnavailable = 1602,

    #[msg("Invalid price data")]
    InvalidPriceData = 1603,

    #[msg("Oracle update required")]
    OracleUpdateRequired = 1604,

    // Lending and borrowing errors (1700-1799)
    #[msg("Insufficient collateral")]
    InsufficientCollateral = 1700,

    #[msg("Position is underwater")]
    PositionUnderwater = 1701,

    #[msg("Liquidation not allowed")]
    LiquidationNotAllowed = 1702,

    #[msg("Health factor too low")]
    HealthFactorTooLow = 1703,

    #[msg("Borrow limit exceeded")]
    BorrowLimitExceeded = 1704,

    #[msg("Interest accrual failed")]
    InterestAccrualFailed = 1705,

    // Access control errors (1800-1899)
    #[msg("Unauthorized access")]
    Unauthorized = 1800,

    #[msg("Admin privileges required")]
    AdminRequired = 1801,

    #[msg("Emergency mode active")]
    EmergencyMode = 1802,

    #[msg("Function is paused")]
    FunctionPaused = 1803,

    // Time and scheduling errors (1900-1999)
    #[msg("Invalid timestamp")]
    InvalidTimestamp = 1900,

    #[msg("Cooldown period not met")]
    CooldownNotMet = 1901,

    #[msg("Operation expired")]
    OperationExpired = 1902,

    #[msg("Too early to execute")]
    TooEarlyToExecute = 1903,

    // Account and PDA errors (2000-2099)
    #[msg("Invalid PDA derivation")]
    InvalidPda = 2000,

    #[msg("Account already initialized")]
    AccountAlreadyInitialized = 2001,

    #[msg("Account not found")]
    AccountNotFound = 2002,

    #[msg("Invalid account size")]
    InvalidAccountSize = 2003,

    #[msg("PDA bump mismatch")]
    PdaBumpMismatch = 2004,

    // Flash loan errors (2100-2199)
    #[msg("Flash loan not repaid")]
    FlashLoanNotRepaid = 2100,

    #[msg("Flash loan amount too large")]
    FlashLoanAmountTooLarge = 2101,

    #[msg("Flash loan fee not paid")]
    FlashLoanFeeNotPaid = 2102,

    #[msg("Flash loan callback failed")]
    FlashLoanCallbackFailed = 2103,

    // Governance errors (2200-2299)
    #[msg("Proposal not active")]
    ProposalNotActive = 2200,

    #[msg("Already voted")]
    AlreadyVoted = 2201,

    #[msg("Insufficient voting power")]
    InsufficientVotingPower = 2202,

    #[msg("Voting period ended")]
    VotingPeriodEnded = 2203,

    // Generic validation errors (2900-2999)
    #[msg("Invalid input parameter")]
    InvalidInput = 2900,

    #[msg("Operation not supported")]
    OperationNotSupported = 2901,

    #[msg("Feature not implemented")]
    FeatureNotImplemented = 2902,

    #[msg("Contract version mismatch")]
    VersionMismatch = 2903,

    #[msg("Configuration error")]
    ConfigurationError = 2904,
}

impl RustyfiError {
    /// Get error severity level for monitoring
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            // Critical errors that need immediate attention
            RustyfiError::MathOverflow |
            RustyfiError::TokenTransferFailed |
            RustyfiError::FlashLoanNotRepaid |
            RustyfiError::PositionUnderwater => ErrorSeverity::Critical,

            // High priority errors
            RustyfiError::SlippageExceeded |
            RustyfiError::InsufficientLiquidity |
            RustyfiError::StalePriceData |
            RustyfiError::Unauthorized => ErrorSeverity::High,

            // Medium priority errors
            RustyfiError::InvalidAmount |
            RustyfiError::InvalidFeeBps |
            RustyfiError::CooldownNotMet => ErrorSeverity::Medium,

            // Low priority errors
            RustyfiError::InvalidInput |
            RustyfiError::AccountNotFound => ErrorSeverity::Low,

            // Info level
            _ => ErrorSeverity::Info,
        }
    }

    /// Get user-friendly error message
    pub fn user_message(&self) -> &'static str {
        match self {
            RustyfiError::SlippageExceeded => "Price moved too much. Try increasing slippage tolerance.",
            RustyfiError::InsufficientLiquidity => "Not enough tokens in the pool for this trade size.",
            RustyfiError::InvalidAmount => "Please enter a valid amount greater than 0.",
            RustyfiError::InsufficientBalance => "You don't have enough tokens for this transaction.",
            RustyfiError::PriceImpactTooHigh => "This trade would significantly impact the price.",
            _ => "An error occurred. Please try again.",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorSeverity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}