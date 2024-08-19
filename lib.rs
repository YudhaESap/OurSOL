use anchor_lang::prelude::*;

declare_id!("OurSOL");

#[program]
pub mod my_social_nft {
    use super::*;

    pub fn create_nft(ctx: Context<CreateNft>, name: String, uri: String) -> Result<()> {
        let nft = &mut ctx.accounts.nft;
        nft.name = name;
        nft.uri = uri;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateNft<'info> {
    #[account(init)]
    pub nft: ProgramAccount<'info, Nft>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Nft {
    pub name: String,
    pub uri: String,
}