// proof-of-aid: a delivery record co-signed by giver + recipient. PLACEHOLDER (Anchor).
use anchor_lang::prelude::*;
declare_id!("AiD1111111111111111111111111111111111111111");

#[program]
pub mod proof_of_aid {
    use super::*;
    pub fn record(ctx: Context<Record>, need_id: String, item: String) -> Result<()> {
        let a = &mut ctx.accounts.aid;
        a.need_id = need_id; a.item = item; a.ts = Clock::get()?.unix_timestamp;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Record<'info> {
    #[account(init, payer = giver, space = 8 + 256)]
    pub aid: Account<'info, Aid>,
    #[account(mut)] pub giver: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Aid { pub need_id: String, pub item: String, pub ts: i64 }
