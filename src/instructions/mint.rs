use quasar_lang::prelude::*;
use quasar_spl::{Mint, Token};

#[derive(Accounts)]
pub struct MintTokens<'info> {
    pub authority: &'info mut Signer,
    #[account(mut)]
    pub mint: &'info mut Account<Mint>,
    #[account(mut)]
    pub to: &'info mut Account<Token>,
    pub token_program: &'info Program<Token>,
}
