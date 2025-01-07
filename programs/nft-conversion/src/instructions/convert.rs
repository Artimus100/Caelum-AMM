use anchor_lang::prelude::*;
use crate::state::NFTConversionState;

#[derive(Accounts)]

///CHECK: This is ok
pub struct Convert<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub nft_account: AccountInfo<'info>,
    pub conversion_state: Account<'info, NFTConversionState>,
}

pub fn handler(ctx: Context<Convert>, _nft_id: Pubkey) -> Result<()> {
    let state = &mut ctx.accounts.conversion_state;
    state.converted = true;
    Ok(())
}
