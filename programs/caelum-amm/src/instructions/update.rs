use anchor_lang::prelude::*;
use crate::state::BondingCurve;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UpdateParams {
    pub new_price: Option<u64>,
    pub new_reserve: Option<u64>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub bonding_curve: Account<'info, BondingCurve>,
    #[account(mut)]
    pub admin: Signer<'info>,
}

pub fn handler(ctx: Context<Update>, params: UpdateParams) -> Result<()> {
    let bonding_curve = &mut ctx.accounts.bonding_curve;

    if let Some(new_price) = params.new_price {
        bonding_curve.price = new_price;
    }

    if let Some(new_reserve) = params.new_reserve {
        bonding_curve.reserve = new_reserve;
    }

    msg!(
        "Updated bonding curve parameters. Price: {}, Reserve: {}",
        bonding_curve.price,
        bonding_curve.reserve
    );
    Ok(())
}
