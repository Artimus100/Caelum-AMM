use anchor_lang::prelude::*;

#[account]
pub struct NFTConversionState {
    pub converted: bool,
}

impl NFTConversionState {
    pub const LEN: usize = 8 + 1;
}
