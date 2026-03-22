#![cfg_attr(not(test), no_std)]

use quasar_lang::prelude::*;
mod instructions;
use instructions::*;

declare_id!("7oY3XcwXGnonxNs92FrnR7e1Dtvf8pRLExvDgznzWTyU");

#[derive(Accounts)]
pub struct Initialize<'info> {
    pub payer: &'info mut Signer,
    pub system_program: &'info Program<System>,
}

impl<'info> Initialize<'info> {
    #[inline(always)]
    pub fn initialize(&self) -> Result<(), ProgramError> {
        Ok(())
    }
}

#[program]
mod quasar_spl_token {
    use super::*;

    #[instruction(discriminator = 0)]
    pub fn initialize(ctx: Ctx<Initialize>) -> Result<(), ProgramError> {
        ctx.accounts.initialize()
    }
}

#[cfg(test)]
mod tests;
