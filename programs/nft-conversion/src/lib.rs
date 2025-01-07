pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("HQMM3aYxwgV7wtNwWzx34ALb7psP4NRsygp7eZ7hoABv");


#[program]
pub mod nft_conversion {
    use super::*;

    pub fn convert(ctx: Context<Convert>, nft_id: Pubkey) -> Result<()> {
        instructions::convert::handler(ctx, nft_id)
    }

    pub fn verify(ctx: Context<Verify>, metadata: Pubkey) -> Result<()> {
        instructions::verify::handler(ctx, metadata)
    }
}