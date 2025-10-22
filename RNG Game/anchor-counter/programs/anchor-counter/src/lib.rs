use anchor_lang::prelude::*;

declare_id!("CcUMv8RacKYVaXmNyhttz7gUU2wSLMNs4eo2aM3gbRWX");

#[program]
pub mod anchor_counter {
    use super::*;

    pub fn initialize(ctx: Context<InitializeCounter>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        msg!("Counter initialized with value starting from {counter.count}");
        //msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn decrement_counter(ctx: Context<Update>) -> Result<()> {
        ctx.accounts.counter.count = ctx.accounts.counter.count.checked_sub(1).unwrap();
        msg!("Counter incremented: {}", ctx.accounts.counter.count);
        //msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn increment_counter(ctx: Context<Update>) -> Result<()> {
        ctx.accounts.counter.count = ctx.accounts.counter.count.checked_add(1).unwrap();
        msg!("Counter incremented: {}", ctx.accounts.counter.count);
        //msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn set_counter(ctx: Context<Update>, value: u64) -> Result<()> {
        ctx.accounts.counter.count = value;
        msg!("Counter set to: {value}");
        //ctx.accounts.counter.count = value.clone();
        Ok(())
    }

    pub fn close_counter(ctx: Context<Close>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeCounter <'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = 8 + 8, // 8 bytes for discriminator + 8 bytes for u64
        seeds = [b"counter", user.key().as_ref()],
        bump,
    )]
    pub counter: Account<'info, Counter>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update <'info>{
    #[account(
        mut,
        seeds = [b"counter", user.key().as_ref()],
        bump,
    )]
    pub counter: Account<'info, Counter>,
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct Close <'info>{
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        close = user,
        seeds = [b"counter", user.key().as_ref()],
        bump
    )]
    pub counter: Account<'info, Counter>
}

#[account]
pub struct Counter {
    pub count: u64,
}
