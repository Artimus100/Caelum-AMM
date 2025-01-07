use anchor_lang::prelude::*;
use crate::state::BondingCurve;

#[derive(Accounts)]
pub struct Mint<'info> {
    #[account(mut)]
    pub bonding_curve: Account<'info, BondingCurve>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Mint>, amount: u64) -> Result<()> {
    let bonding_curve = &mut ctx.accounts.bonding_curve;

    // Ensure there is enough reserve for the requested mint amount
    if bonding_curve.reserve < amount {
        return err!(crate::error::BondingCurveError::InsufficientLiquidity);
    }

    // Update reserve and price based on bonding curve logic
    bonding_curve.reserve -= amount;
    bonding_curve.price += amount / 10; // Example: linear price increase

    msg!("Minted {} tokens successfully. New price: {}", amount, bonding_curve.price);
    Ok(())
}
