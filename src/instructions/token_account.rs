use quasar_lang::prelude::*;
use quasar_spl::token_interface::{Mint, TokenInterface};

#[derive(Accounts)]
pub struct TokenAccount<'info> {
    pub user: &'info mut Signer,
    #[account(mut, seeds = [b"vault", user], bump)]
    pub vault: &'info mut UncheckedAccount,
    pub system_program: &'info Program<System>,
}

impl<'info> TokenAccount<'info> {
    #[inline(always)]
    pub fn deposit(&self, amount: u64) -> Result<(), ProgramError> {
        self.system_program
            .transfer(self.user, self.vault, amount)
            .invoke()
    }
}
