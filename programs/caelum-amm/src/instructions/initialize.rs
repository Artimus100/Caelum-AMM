use anchor_lang::prelude::*;
use crate::state::BondingCurve;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitializeParams {
    pub initial_price: u64,
    pub reserve_amount: u64,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = BondingCurve::LEN)]
    pub bonding_curve: Account<'info, BondingCurve>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Initialize>, params: InitializeParams) -> Result<()> {
    let bonding_curve = &mut ctx.accounts.bonding_curve;
    bonding_curve.price = params.initial_price;
    bonding_curve.reserve = params.reserve_amount;
    Ok(())
}
