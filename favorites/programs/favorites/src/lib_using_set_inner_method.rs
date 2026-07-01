
use anchor_lang::prelude::*;

declare_id!("ENuWjmte9BG9YFG5TE2xcZBkhEme6GesosJTMH9ZG5Ls");  // <-- added !

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;


#[program]
pub mod favorites_using_set_inner {
    use super::*;

    pub fn set_favorite(
        ctx: Context<SetFavorites>,   // <-- fixed typo
        number: u64,
        color: String,
        hobbies: Vec<String>,
    ) -> Result<()> {
        msg!("Greeting from {}", ctx.program_id);
        let user_public_key = ctx.accounts.user.key();
        msg!(
            "User {user_public_key}'s favorite number is {number}, favorite color is: {color}, and their hobbies are {hobbies:?}",
        );

        msg!("set_inner() is a helper method provided by Anchor on the Account<'info, T> wrapper.
Its job is simple: completely replace the data payload of an on‑chain account with a new struct, while safely preserving the 8‑byte Anchor discriminator.");
        ctx.accounts.favorites.set_inner(Favorites {
            number,
            color,
            hobbies,
        });
        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct Favorites {
    pub number: u64,
    #[max_len(50)]
    pub color: String,
    #[max_len(5, 50)]
    pub hobbies: Vec<String>,
}

#[derive(Accounts)]
pub struct SetFavorites<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init_if_needed,
        payer = user,
        space = ANCHOR_DISCRIMINATOR_SIZE + Favorites::INIT_SPACE,
        seeds = [b"favorites", user.key().as_ref()],
        bump
    )]
    pub favorites: Account<'info, Favorites>,
    pub system_program: Program<'info, System>,
}


