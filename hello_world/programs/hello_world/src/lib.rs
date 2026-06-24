use anchor_lang::prelude::*;

declare_id!("CexMsYyq6E5un6cw12AH6r2Fu2oCeENc6GXESJN6oK9r");

#[program]
pub mod hello_world{
    use super::*;

    pub fn say_hello(ctx:Context<SayHello>)-> Result<()>{
        msg!("Hello, Solana!");

        msg!("Called by: {:?}", ctx.accounts.user.key());

        Ok(())
    }
}


#[derive(Accounts)]
pub struct SayHello<'info>{
    #[account(mut)]
   pub user: Signer<'info>
}