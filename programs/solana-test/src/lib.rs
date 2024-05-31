use anchor_lang::prelude::*;

declare_id!("6fnhTz8Y8QNyLy5cjge8QvdyiLw3PPSBxDKuQg5NyUPJ");

#[program]
pub mod solana_test {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
#[account]
pub struct Message {
    pub author: Pubkey,
    pub timestamp: i64,
    pub content: String,
}
