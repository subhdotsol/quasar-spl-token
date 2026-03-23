use quasar_lang::prelude::*;
use quasar_spl::{Mint, Token};

#[derive(Accounts)]
pub struct CreateTokenAccount<'info> {
    pub payer: &'info mut Signer,
    pub owner: &'info mut UncheckedAccount,
    pub mint: &'info Account<Mint>,
    #[account(
        init,
        payer = payer,
        token::mint = mint,
        token::authority = owner,
    )]
    pub token_account: &'info mut Account<Token>,
    pub rent: &'info Sysvar<Rent>,
    pub token_program: &'info Program<Token>,
    pub system_program: &'info Program<System>,
}
