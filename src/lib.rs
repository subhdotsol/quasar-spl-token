#![cfg_attr(not(test), no_std)]

use quasar_lang::prelude::*;
mod instructions;
use instructions::*;

declare_id!("7oY3XcwXGnonxNs92FrnR7e1Dtvf8pRLExvDgznzWTyU");

#[program]
mod quasar_spl_token {
    use super::*;

    #[instruction(discriminator = 0)]
    pub fn create_mint(ctx: Ctx<CreateMint>) -> Result<(), ProgramError> {
        ctx.accounts.create_mint()
    }

    #[instruction(discriminator = 1)]
    pub fn mint_tokens(ctx: Ctx<MintTokens>, amount: u64) -> Result<(), ProgramError> {
        ctx.accounts.mint_tokens(amount)
    }

    #[instruction(discriminator = 2)]
    pub fn create_token_account(ctx: Ctx<CreateTokenAccount>) -> Result<(), ProgramError> {
        ctx.accounts.create_token_account()
    }

    #[instruction(discriminator = 3)]
    pub fn transfer(ctx: Ctx<TransferTokens>, amount: u64) -> Result<(), ProgramError> {
        ctx.accounts.transfer_tokens(amount)
    }
}

#[cfg(test)]
mod tests;
