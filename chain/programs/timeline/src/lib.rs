// timeline: immutable event/evidence log. PLACEHOLDER.
use anchor_lang::prelude::*;
declare_id!("TimE111111111111111111111111111111111111111");

#[program]
pub mod timeline {
    use super::*;
    pub fn commit(_ctx: Context<Commit>, _hash: [u8; 32]) -> Result<()> { Ok(()) }
}
#[derive(Accounts)] pub struct Commit<'info> { #[account(mut)] pub signer: Signer<'info> }
