use anchor_lang::prelude::*;

#[account]
pub struct BondingCurve {
    pub price: u64,
    pub reserve: u64,
}

impl BondingCurve {
    pub const LEN: usize = 8 + 8 + 8;
}
