pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("7aa35rvHUZ3BZfnkg9j51f8CMEMf9hC9etxiZiAdwD6E");

#[program]
pub mod swap {
    use crate::instruction::MakeOffer;

    use super::*;

    pub fn make_offer(ctx: Context<MakeOffer>) -> Result<()> 
    {
        instructions::make_offer::send_offered_tokens_to_vault ()?;
        instructions::make_offer::save_offer()    
    }
}