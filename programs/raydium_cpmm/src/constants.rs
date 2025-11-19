use anchor_lang::prelude::*;

pub mod seeds {
    pub const POOL_STATE: &[u8] = b"pool_state";
    pub const POOL_AUTHORITY:[u8] = b"pool_authority";
    pub const POOL_VAULT_A:&[u8]= b"pool_vault_a";
    pub const POOL_VAULT_B:&[u8]= b"pool_vault_b";
    pub const LP_MINT:&[u8]= b"lp_mint";
    pub const USER_POSITION:&[u8]= b"user_position";
     
}

pub mod fees {
    pub const TRADING_FEE_BPS:u64 = 30; // 0.3%
    pub const PROTOCOL_FEE_BPS:u64 = 5; // 0.05%
    pub const BPS_DENOMINATOR:u64 = 10_000;
}

pub mod limits {
    pub const MINIMUM_LIQUIDITY:u64 = 1_000; // Minimum liquidity to avoid division by zero
    pub const MAX_SLIPPAGE_BPS:u64 = 1_000; // 10%
    pub const MIN_TOKEN_AMOUNT:u64 = 1; // Minimum token amount to prevent dust
}

pub mod time {
    pub const MIN_LOCK_DURATION:i64 = 3_600; // 1 hour in seconds
    pub const MAX_LOCK_DURATION:i64 = 31_536_000; // 1 year in seconds
}