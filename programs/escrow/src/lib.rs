use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

pub use state::*;
pub use instructions::*;

declare_id!("2aMw9v1c5uH4ANd8HFjwrhqgLdjcLPYUAaWSQsigvN94");

#[program]
pub mod escrow {
    use super::*;

    pub fn make(ctx: Context<Make>, seed: u64, deposit: u64, receive: u64) -> Result<()> {
        ctx.accounts.init_escrow(seed, receive, &ctx.bumps)?;
        ctx.accounts.deposit(deposit)
    }

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
