pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("7uTEXZNdMUkjBmN5LvhN7iZ56WsgoW5PjTDBgGvqiFMT");

#[program]
pub mod caelum_amm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, params: InitializeParams) -> Result<()> {
        instructions::initialize::handler(ctx, params)
    }

    pub fn mint(ctx: Context<Mint>, amount: u64) -> Result<()> {
        instructions::mint::handler(ctx, amount)
    }

    pub fn update(ctx: Context<Update>, params: UpdateParams) -> Result<()> {
        instructions::update::handler(ctx, params)
    }
}
