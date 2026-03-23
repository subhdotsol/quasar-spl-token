use quasar_lang::prelude::*;
use quasar_spl::{Mint, Token, TokenCpi};

#[derive(Accounts)]
pub struct MintTokens<'info> {
    pub authority: &'info mut Signer,
    #[account(mut)]
    pub mint: &'info mut Account<Mint>,
    #[account(mut)]
    pub to: &'info mut Account<Token>,
    pub token_program: &'info Program<Token>,
}

impl<'info> MintTokens<'info> {
    #[inline(always)]
    pub fn mint_tokens(&self, amount: u64) -> Result<(), ProgramError> {
        self.token_program
            .mint_to(self.mint, self.to, self.authority, amount)
            .invoke()
    }
}
