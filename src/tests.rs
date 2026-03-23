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

#[test]
fn test_create_mint() {
    let mut svm = setup();

    let payer = Pubkey::new_unique();
    svm.airdrop(&payer, 10_000_000_000);

    // Derive the mint PDA
    let (mint_pda, _bump) =
        Pubkey::find_program_address(&[b"mint", payer.as_ref()], &Pubkey::from(crate::ID));

    let instruction: Instruction = CreateMintInstruction {
        payer: to_address(&payer),
        mint: to_address(&mint_pda),
        token_program: to_address(&TOKEN_PROGRAM_ID),
        system_program: to_address(&quasar_svm::system_program::ID),
    }
    .into();

    // Pass both payer and mint PDA (uninitialized) as accounts
    let result = svm.process_instruction(
        &instruction,
        &[
            Account {
                address: payer,
                lamports: 10_000_000_000,
                data: vec![],
                owner: quasar_svm::system_program::ID,
                executable: false,
            },
            Account {
                address: mint_pda,
                lamports: 0,
                data: vec![],
                owner: quasar_svm::system_program::ID,
                executable: false,
            },
        ],
    );

    result.assert_success();

    // Verify the mint was created
    let mint_account = result.account(&mint_pda).expect("mint account not found");
    assert_eq!(
        mint_account.owner, TOKEN_PROGRAM_ID,
        "mint owner should be token program"
    );

    // Verify mint state
    let mint_state = Mint::unpack(&mint_account.data).expect("failed to unpack mint");
    assert_eq!(mint_state.decimals, 6, "mint decimals should be 6");
    assert!(mint_state.is_initialized, "mint should be initialized");
    assert_eq!(mint_state.supply, 0, "initial supply should be 0");
}
