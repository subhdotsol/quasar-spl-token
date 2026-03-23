extern crate std;

use quasar_svm::{
    token::{self, Mint, TokenAccount},
    Account, Instruction, Pubkey, QuasarSvm,
};
use solana_address::Address;
use solana_program_pack::Pack;

use quasar_spl_token_client::{
    CreateMintInstruction, CreateTokenAccountInstruction, MintTokensInstruction,
    TransferInstruction,
};

const TOKEN_PROGRAM_ID: Pubkey =
    solana_pubkey::pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

fn setup() -> QuasarSvm {
    let elf = include_bytes!("../target/deploy/quasar_spl_token.so");
    QuasarSvm::new().with_program(&Pubkey::from(crate::ID), elf)
}

fn to_address(pubkey: &Pubkey) -> Address {
    Address::from(pubkey.to_bytes())
}

/// Mark specific account indices as signers on an instruction.
fn with_signers(mut ix: Instruction, indices: &[usize]) -> Instruction {
    for &i in indices {
        ix.accounts[i].is_signer = true;
    }
    ix
}
