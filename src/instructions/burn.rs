use quasar_lang::prelude::*;
use quasar_spl::{Mint, Token, TokenCpi};

#[derive(Accounts)]
pub struct BurnTokens<'info> {
    pub authority: &'info mut Signer,
    #[account(mut)]
    pub from: &'info mut Account<Token>,
    #[account(mut)]
    pub mint: &'info mut Account<Mint>,
    pub token_program: &'info Program<Token>,
}

impl<'info> BurnTokens<'info> {
    #[inline(always)]
    pub fn burn_tokens(&self, amount: u64) -> Result<(), ProgramError> {
        self.token_program
            .burn(self.from, self.mint, self.authority, amount)
            .invoke()
    }
}
