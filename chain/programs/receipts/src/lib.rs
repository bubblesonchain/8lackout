// receipts: compressed (cNFT) aid receipts at scale. PLACEHOLDER.
use anchor_lang::prelude::*;
declare_id!("Rcpt111111111111111111111111111111111111111");

#[program]
pub mod receipts {
    use super::*;
    pub fn mint(_ctx: Context<Mint>, _uri: String) -> Result<()> { Ok(()) }
}
#[derive(Accounts)] pub struct Mint<'info> { #[account(mut)] pub payer: Signer<'info> }
