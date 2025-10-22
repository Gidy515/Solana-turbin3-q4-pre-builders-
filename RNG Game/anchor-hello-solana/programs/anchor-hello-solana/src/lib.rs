use anchor_lang::prelude::*;

declare_id!("3ZFuTfa2WY6dyNM9Y9CfQqAhbNWrd1Y3WqR4LSWrP9xY");

#[program]
pub mod anchor_hello_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
