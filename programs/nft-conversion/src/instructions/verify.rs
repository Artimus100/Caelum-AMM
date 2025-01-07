use anchor_lang::prelude::*;
use crate::state::NFTConversionState;

#[derive(Accounts)]
pub struct Verify<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub nft_metadata: AccountInfo<'info>, // NFT metadata account
    #[account(mut)]
    pub conversion_state: Account<'info, NFTConversionState>,
}

pub fn handler(ctx: Context<Verify>, metadata: Pubkey) -> Result<()> {
    let state = &mut ctx.accounts.conversion_state;

    // Simulated metadata verification logic
    if metadata != ctx.accounts.nft_metadata.key() {
        return err!(crate::error::NFTConversionError::InvalidMetadata);
    }

    // Mark the NFT as verified in the conversion state
    state.converted = true;

    msg!("NFT metadata verified successfully for: {}", metadata);
    Ok(())
}
