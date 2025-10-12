use anchor_lang::prelude::*;

declare_id!("B9fBptXaatjWo35uJ8yfN3gajazdtf7yyyRaffhzqTa4");

#[program]
pub mod hello_world_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
