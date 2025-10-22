use anchor_lang::prelude::*;

declare_id!("CcUMv8RacKYVaXmNyhttz7gUU2wSLMNs4eo2aM3gbRWX");

#[program]
pub mod anchor_counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
