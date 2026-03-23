use quasar_lang::prelude::*;
use quasar_spl::{Token, TokenCpi};

#[derive(Accounts)]
pub struct TransferTokens<'info> {
    pub authority: &'info mut Signer,
    #[account(mut)]
    pub from: &'info mut Account<Token>,
    #[account(mut)]
    pub to: &'info mut Account<Token>,
    pub token_program: &'info Program<Token>,
}

impl<'info> TransferTokens<'info> {
    #[inline(always)]
    pub fn transfer_tokens(&self, amount: u64) -> Result<(), ProgramError> {
        self.token_program
            .transfer(self.from, self.to, self.authority, amount)
            .invoke()
    }
}
