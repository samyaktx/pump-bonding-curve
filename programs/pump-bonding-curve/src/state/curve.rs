use anchor_lang::prelude::*;


#[account]
pub struct CurveConfig {
    pub fee: f64,
}

impl CurveConfig {
    pub const SEED: &'static str = "curve_config";

    // Discriminator 8 + f64 (32) + 8 = 48
    pub const ACCOUNT_SIZE: usize = 8 + 32 + 8;

    pub fn new(fee: f64) -> Self {
        Self { fee }
    }
}
