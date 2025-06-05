use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount};

declare_id!("YourProgramIdHere1111111111111111111111111111");

#[program]
pub mod oursol {
    use super::*;

    pub fn mint_soul_token(ctx: Context<MintSoulToken>) -> Result<()> {
        // Mint 1 token to user
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.user_token_account.to_account_info(),
                authority: ctx.accounts.mint_authority.to_account_info(),
            },
        );
        token::mint_to(cpi_ctx, 1)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintSoulToken<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,

    /// CHECK: authority must sign and match mint authority
    #[account(signer)]
    pub mint_authority: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
}