use anchor_lang::prelude::*;

#[error_code]
pub enum NFTConversionError {
    #[msg("Invalid NFT metadata.")]
    InvalidMetadata,
    #[msg("NFT conversion failed.")]
    ConversionFailed,
}
