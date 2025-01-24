use anchor_lang::prelude::*;

declare_id!("7aa35rvHUZ3BZfnkg9j51f8CMEMf9hC9etxiZiAdwD6E");

#[program]
pub mod swap {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
