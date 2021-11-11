use anchor_lang::prelude::*;

declare_id!("EyJzqBLX1QCjfyW6E4hz4aAnrHPgPkaEwposrA9ewZ3T");

#[program]
pub mod myepicproject {
    use super::*;

    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
        // Get a reference to the account.
        let base_account = &mut ctx.accounts.base_account;
        // Initialize total_videos.
        base_account.total_videos = 0;
        // Initialise video_list
        base_account.video_list = Vec::new();
        
        Ok(())
    }

    // The function now accepts a video_link param from the user. We also reference the user from the Context
    pub fn add_video(ctx: Context<AddVideo>, video_link: String) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        // Build the struct.
        let item = ItemStruct {
            video_id: base_account.total_videos as u32,
            video_link: video_link.to_string(),
            user_address: *user.to_account_info().key,
            likes: 0u32,
            liked_list: Vec::new(),
        };
            
        // Add it to the video_list vector.
        base_account.video_list.push(item);
        base_account.total_videos += 1;
        Ok(())
    }

    pub fn like_video(ctx: Context<AddVideo>, video_id: u32) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        // If liked_list doesn't contain the user's address
        if !base_account.video_list[video_id as usize].liked_list.contains(&*user.to_account_info().key) {
            // Add the user's address to the liked_list
            base_account.video_list[video_id as usize].liked_list.push(*user.to_account_info().key);
            // Add one to the likes
            base_account.video_list[video_id as usize].likes += 1;
        } else {
            // If liked_list does contain the user's address
            // Remove the user's address from the liked_list
            base_account.video_list[video_id as usize].liked_list.retain(|&x| x != *user.to_account_info().key);
            // Subtract one from the likes
            base_account.video_list[video_id as usize].likes -= 1;
        }
        
        Ok(())
    }
}

// Attach certain variables to the StartStuffOff context.
#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}

// Specify what data you want in the AddVideo Context.
// Getting a handle on the flow of things :)?
#[derive(Accounts)]
pub struct AddVideo<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

// Create a custom struct for us to work with.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub video_id: u32,
    pub video_link: String,
    pub user_address: Pubkey,
    pub likes: u32,
    pub liked_list: Vec<Pubkey>,
}

// Tell Solana what we want to store on this account.
#[account]
pub struct BaseAccount {
    pub total_videos: u64,
    pub video_list: Vec<ItemStruct>,
}