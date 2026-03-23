use quasar_lang::prelude::*;
use quasar_spl::Token;

#[derive(Accounts)]
pub struct TransferTokens<'info> {
    pub authority: &'info mut Signer,
    #[account(mut)]
    pub from: &'info mut Account<Token>,
    #[account(mut)]
    pub to: &'info mut Account<Token>,
    pub token_program: &'info Program<Token>,
}
