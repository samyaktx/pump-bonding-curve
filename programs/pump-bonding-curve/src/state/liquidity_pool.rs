use anchor_lang::prelude::*;

#[account]
pub struct LiquidityProvider {
    pub num_of_shares: u64,   // number of shares this provider holds in the liquidity pool ( didn't add to contract now )
}

impl LiquidityProvider {
    pub const SEED_PREFIX: &'static str = "liquidity_provider";

    // Discriminator 8 + u64 (8) = 16
    pub const ACCOUNT_SIZE: usize = 8 + 8;
}
