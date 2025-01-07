use anchor_lang::prelude::*;

#[error_code]
pub enum BondingCurveError {
    #[msg("Invalid input parameters.")]
    InvalidInput,
    #[msg("Insufficient liquidity.")]
    InsufficientLiquidity,
}
