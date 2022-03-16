use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod blog {
    use super::*;
    pub fn init_blog(ctx: Context<InitBlog>) -> ProgramResult {
        // get accounts from ctx
        let blog_account = &mut ctx.accounts.blog_account;
        let genesis_post_account = &mut ctx.accounts.genesis_post_account;
        let authority = &mut ctx.accounts.authority;

        // sets the blog state
        blog_account.authority = authority.key();
        blog_account.current_post_key = genesis_post_account.key();

        Ok(())
    }
    pub fn signup_user(ctx: Context<SignupUser>, name: String, avatar: String) -> ProgramResult {
        let user_account = &mut ctx.accounts.user_account;
        let authority = &mut ctx.accounts.authority;
        user_account.name = name;
        user_account.avatar = avatar;
        user_account.authority = authority.key();
        Ok(())
    }

    pub fn update_user(ctx: Context<UpdateUser>, name: String, avatar: String) -> ProgramResult {
        let user_account = &mut ctx.accounts.user_account;
        user_account.name = name;
        user_account.avatar = avatar;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitBlog {
    #[account(init, payer = authority, space = 8+32+32)]
    pub blog_account: Account<'info, BlogState>,
    #[account(init, payer = authority, space = 8+32+32+32+32+8)]
    pub genesis_post_account: Account<'info, PostState>,
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SignupUser<'info> {
    #[account(init, payer = authority, space = 8 + 40 + 120  + 32)]
    pub user_account: Account<'info, UserState>,
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateUser<'info> {
    #[account(mut, has_one = authority)]
    pub user_account: Account<'info, UserState>,
    pub authority: Signer<'info>,
}
#[account]
pub struct BlogState {
    pub current_post_key: Pubkey,
    pub authority: Pubkey,
}
#[account]
pub struct UserState {
    pub name: String,
    pub avatar: String,
    pub authority: Pubkey,
}
