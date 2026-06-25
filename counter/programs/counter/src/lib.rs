use anchor_lang::prelude::*;

declare_id!("DnhGSQsfDTmkaWH2wyb6BscZnNqdRdpETpdeNjJDFC46");

#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.value = 0;
        Ok(())
    }

    pub fn increment(ctx: Context<UpdateCounter>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        require!(counter.value < u64::MAX, MyError::Overflow);
        counter.value += 1;
        Ok(())
    }

    pub fn decrement(ctx: Context<UpdateCounter>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        require!(counter.value > 0, MyError::Underflow);
        counter.value -= 1;
        Ok(())
    }

    pub fn set(ctx: Context<UpdateCounter>, new_value: u64) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.value = new_value;
        Ok(())
    }

    pub fn reset(ctx: Context<UpdateCounter>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.value = 0;
        Ok(())
    }
}

#[error_code]
pub enum MyError {
    #[msg("Counter overflow")]
    Overflow,
    #[msg("Counter underflow")]
    Underflow,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        seeds = [b"counter"],
        bump,
        payer = user,
        space = 8 + 8,
    )]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateCounter<'info> {
    #[account(
        mut,
        seeds = [b"counter"],
        bump,
    )]
    pub counter: Account<'info, Counter>,
    pub user: Signer<'info>,
}

#[account]
pub struct Counter {
    pub value: u64,
}