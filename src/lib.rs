#![cfg_attr(not(test), no_std)]

use quasar_lang::prelude::*;
mod instructions;
use instructions::*;

declare_id!("7oY3XcwXGnonxNs92FrnR7e1Dtvf8pRLExvDgznzWTyU");

#[program]
mod quasar_spl_token {
    use super::*;
}
