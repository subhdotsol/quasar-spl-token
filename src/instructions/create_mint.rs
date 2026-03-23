use quasar_lang::prelude::*;
use quasar_spl::{Mint, Token};

#[derive(Accounts)]
pub struct CreateMint<'info> {
    pub payer: &'info mut Signer,
    #[account(
        init,
        payer = payer,
        mint::decimals = 6,
        mint::authority = payer,
        seeds = [b"mint", payer],
        bump,
    )]
    pub mint: &'info mut Account<Mint>,
    pub token_program: &'info Program<Token>,
    pub system_program: &'info Program<System>,
}

impl<'info> CreateMint<'info> {
    #[inline(always)]
    pub fn create_mint(&self) -> Result<(), ProgramError> {
        // #[account(init, mint::decimals, mint::authority)] handles
        // the CreateAccount + InitializeMint2 CPIs automatically.
        Ok(())
    }
}
