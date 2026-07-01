use anchor_lang::prelude::*;

declare_id!("ENuWjmte9BG9YFG5TE2xcZBkhEme6GesosJTMH9ZG5Ls");

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;


// ============================================================
// 1. PROGRAM LOGIC (Instructions)
// ============================================================

#[program]
pub mod favorites {
    use super::*;

    // --- CREATE: First-time setup ---
    pub fn initialize(
        ctx: Context<Initialize>,
        number: u64,
        color: String,
        hobbies: Vec<String>,
    ) -> Result<()> {
        // ✅ Input validation – reject empty or garbage data
        require!(!color.is_empty(), FavoritesError::EmptyColor);
        require!(!hobbies.is_empty(), FavoritesError::NoHobbies);
        for hobby in &hobbies {
            require!(!hobby.is_empty(), FavoritesError::EmptyHobby);
        }

        // Get the user's public key
        let user_key = ctx.accounts.user.key();
        let bump = ctx.bumps.favorites; // Anchor gives us the bump

        // Write everything in one atomic operation using set_inner()
        ctx.accounts.favorites.set_inner(Favorites {
            owner: user_key,
            bump,               // Store the bump for future updates
            number,
            color,
            hobbies,
        });

        msg!("✅ Favorites initialized for user: {}", user_key);

        Ok(())
    }

    // --- UPDATE: Modify existing data ---
    pub fn update(
        ctx: Context<Update>,
        number: u64,
        color: String,
        hobbies: Vec<String>,
    ) -> Result<()> {
        // ✅ Input validation – same checks
        require!(!color.is_empty(), FavoritesError::EmptyColor);
        require!(!hobbies.is_empty(), FavoritesError::NoHobbies);
        for hobby in &hobbies {
            require!(!hobby.is_empty(), FavoritesError::EmptyHobby);
        }

        // Get the existing account to preserve owner and bump
        let existing = &ctx.accounts.favorites;
        let owner = existing.owner;
        let bump = existing.bump;

        // Replace the entire content using set_inner()
        ctx.accounts.favorites.set_inner(Favorites {
            owner,              // Keep the original owner
            bump,               // Keep the stored bump
            number,
            color,
            hobbies,
        });

        msg!("✅ Favorites updated for user: {}", ctx.accounts.user.key());

        Ok(())
    }

    // --- CLOSE: Delete the account and refund rent ---
    pub fn close(ctx: Context<Close>) -> Result<()> {
        msg!("✅ Favorites account closed for user: {}", ctx.accounts.user.key());
        Ok(())
    }
}

// ============================================================
// 2. CUSTOM ERRORS
// ============================================================

#[error_code]
pub enum FavoritesError {
    #[msg("Color cannot be empty")]
    EmptyColor,
    #[msg("You must have at least one hobby")]
    NoHobbies,
    #[msg("Hobby cannot be an empty string")]
    EmptyHobby,
}




// ============================================================
// 3. CONTEXT STRUCTS (Account Validation)
// ============================================================

// Context for CREATING a new Favorites account
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,                 // Only creates – fails if account already exists
        payer = user,
        space = ANCHOR_DISCRIMINATOR_SIZE + Favorites::INIT_SPACE,
        seeds = [b"favorites", user.key().as_ref()],
        bump
    )]
    pub favorites: Account<'info, Favorites>,

    pub system_program: Program<'info, System>,
}

// Context for UPDATING an existing Favorites account
#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"favorites", user.key().as_ref()],
        bump = favorites.bump,   // Use the stored bump
        has_one = user           // 🔒 Double-check: favorites.owner == user.key()
    )]
    pub favorites: Account<'info, Favorites>,
}

// Context for CLOSING (deleting) a Favorites account and refunding rent
#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"favorites", user.key().as_ref()],
        bump = favorites.bump,
        has_one = user,
        close = user            // 🔥 Refunds all lamports to 'user' and deletes the account
    )]
    pub favorites: Account<'info, Favorites>,
}



// ============================================================
// 4. ACCOUNT STRUCT (On-chain Data)
// ============================================================

#[account]
#[derive(InitSpace)]
pub struct Favorites {
    pub owner: Pubkey,        // Explicitly track the owner
    pub bump: u8,             // Store the PDA bump for efficient re-derivation
    pub number: u64,

    #[max_len(50)]
    pub color: String,

    #[max_len(5, 50)]
    pub hobbies: Vec<String>,
}
