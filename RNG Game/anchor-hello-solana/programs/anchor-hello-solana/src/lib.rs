use anchor_lang::prelude::*;

declare_id!("3ZFuTfa2WY6dyNM9Y9CfQqAhbNWrd1Y3WqR4LSWrP9xY");

#[program]
pub mod anchor_hello_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        msg!("Hello, Solana!");
        // Access the signer’s public key
        msg!("This message was sent by: {:?}", ctx.accounts.user.key());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize <'info>{
    #[account(mut)]
    pub user: Signer<'info>,
}
