use quasar_lang::prelude::*;
use quasar_spl::{Mint, Token, TokenCpi, InitMint};

#[derive(Accounts)]
pub struct CreateMint<'info> {
    // The payer who funds the mint account creation
    #[account(signer, writable)]
    pub payer: &'info AccountInfo<'info>,

    // The mint PDA we're creating
    // seeds + bump determine the PDA address
    #[account(
        init,
        payer = payer,
        mint::decimals = 6,
        mint::authority = payer,
        seeds = [b"mint", payer.key().as_ref()],
        bump,
    )]
    pub mint: &'info mut Account<'info, Mint>,

    // SPL Token program — required for mint initialization
    pub token_program: Program<'info, Token>,

    // System program — required for account creation
    pub system_program: Program<'info, System>,

    // Rent sysvar (optional — can fetch via syscall)
    pub rent: Sysvar<'info, Rent>,
}; 


impl<'info> CreateMint<'info> {
    #[inline(always)]
    pub fn deposit(&self, amount: u64) -> Result<(), ProgramError> {
        self.system_program
            .transfer(self.user, self.vault, amount)
            .invoke()
    }
}

pub fn process_create_mint(
    ctx: Context<CreateMint>,
    decimals: u8,
) -> ProgramResult {
    // When using #[account(init, mint::decimals, mint::authority)]
    // the framework calls InitializeMint2 automatically.
    // Nothing extra to do here unless you need custom logic.

    // You can read state immediately after init:
    let supply = ctx.accounts.mint.supply();        // 0
    let dec    = ctx.accounts.mint.decimals();      // = decimals param
    let auth   = ctx.accounts.mint.mint_authority(); // Some(&payer)

    Ok(())
}
