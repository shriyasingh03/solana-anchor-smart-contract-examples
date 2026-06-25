use anchor_lang::prelude::*;

declare_id!("DnhGSQsfDTmkaWH2wyb6BscZnNqdRdpETpdeNjJDFC46");

#[program]
pub mod counter{
    use super::*;

    //initialize
    pub fn initialize(ctx: Context<Initialize>)-> Result<()>{
        let counter = &mut ctx.accounts.counter;
        counter.value = 0;
        Ok(())
    }



    //Increase by 1 
    pub fn increment(ctx: Context<UpdateCounter>)-> Result<()>{
        let counter = &mut ctx.accounts.counter;
        counter.value = counter.value.checked_add(1).unwrap();
        Ok(())
    }


    //decrease by 1
    pub fn decrement(ctx: Context<UpdateCounter>)-> Result<()>{
        let counter = &mut ctx.accounts.counter;
        counter.value = counter.value.checked_sub(1).unwrap();
        Ok(())
    }




    // set to any value : u64
    pub fn set(ctx: Context<UpdateCounter>, new_value: u64)-> Result<()>{
        let counter = &mut ctx.accounts.counter;
        counter.value = new_value;
        Ok(())
     }



    //reset to 0
pub fn reset(ctx: Context<UpdateCounter>)->Result<()>{
    let counter = &mut ctx.accounts.counter;
    counter.value = 0;
    Ok(())
}
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        seeds = [b"counter"],
        bump,
        payer =  user,
        space =  8 + 8,
    )]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program : Program<'info, System>,
}


#[derive(Accounts)]
pub struct UpdateCounter<'info>{
    #[account(
        mut,
        seeds = [b"counter"],
        bump,
    )]

    pub counter: Account<'info, Counter>,
    pub user: Signer<'info>,
   
}


#[account]
pub struct Counter{
    pub value: u64
}