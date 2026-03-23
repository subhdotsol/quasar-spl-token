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

#[test]
fn test_create_token_account() {
    let mut svm = setup();

    let payer = Pubkey::new_unique();
    let owner = Pubkey::new_unique();
    let mint = Pubkey::new_unique();
    let token_account = Pubkey::new_unique();

    svm.airdrop(&payer, 10_000_000_000);

    // Pre-create a mint account
    let mint_state = Mint {
        mint_authority: solana_program_option::COption::Some(payer),
        supply: 0,
        decimals: 6,
        is_initialized: true,
        freeze_authority: solana_program_option::COption::None,
    };
    svm.set_account(token::create_keyed_mint_account(&mint, &mint_state));

    let rent_sysvar = solana_pubkey::pubkey!("SysvarRent111111111111111111111111111111111");

    let instruction: Instruction = with_signers(
        CreateTokenAccountInstruction {
            payer: to_address(&payer),
            owner: to_address(&owner),
            mint: to_address(&mint),
            token_account: to_address(&token_account),
            rent: to_address(&rent_sysvar),
            token_program: to_address(&TOKEN_PROGRAM_ID),
            system_program: to_address(&quasar_svm::system_program::ID),
        }
        .into(),
        &[3], // token_account needs to be a signer for create_account CPI
    );

    // Pass payer, owner, and token_account (uninitialized) as accounts
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
                address: token_account,
                lamports: 0,
                data: vec![],
                owner: quasar_svm::system_program::ID,
                executable: false,
            },
        ],
    );

    result.assert_success();

    // Verify token account was created
    let ta = result
        .account(&token_account)
        .expect("token account not found");
    assert_eq!(
        ta.owner, TOKEN_PROGRAM_ID,
        "token account owner should be token program"
    );

    // Verify token account state
    let ta_state = TokenAccount::unpack(&ta.data).expect("failed to unpack token account");
    assert_eq!(ta_state.mint, mint, "token account mint mismatch");
    assert_eq!(ta_state.owner, owner, "token account owner mismatch");
    assert_eq!(ta_state.amount, 0, "initial balance should be 0");
}

#[test]
fn test_mint_tokens() {
    let mut svm = setup();

    let authority = Pubkey::new_unique();
    let mint = Pubkey::new_unique();
    let to = Pubkey::new_unique();

    svm.airdrop(&authority, 10_000_000_000);

    // Pre-create mint account with authority as mint_authority
    let mint_state = Mint {
        mint_authority: solana_program_option::COption::Some(authority),
        supply: 0,
        decimals: 6,
        is_initialized: true,
        freeze_authority: solana_program_option::COption::None,
    };
    svm.set_account(token::create_keyed_mint_account(&mint, &mint_state));

    // Pre-create destination token account
    let token_state = TokenAccount {
        mint,
        owner: authority,
        amount: 0,
        state: spl_token_interface::state::AccountState::Initialized,
        ..TokenAccount::default()
    };
    svm.set_account(token::create_keyed_token_account(&to, &token_state));

    let mint_amount: u64 = 1_000_000;

    let instruction: Instruction = MintTokensInstruction {
        authority: to_address(&authority),
        mint: to_address(&mint),
        to: to_address(&to),
        token_program: to_address(&TOKEN_PROGRAM_ID),
        amount: mint_amount,
    }
    .into();

    let result = svm.process_instruction(
        &instruction,
        &[Account {
            address: authority,
            lamports: 10_000_000_000,
            data: vec![],
            owner: quasar_svm::system_program::ID,
            executable: false,
        }],
    );

    result.assert_success();

    // Verify tokens were minted to destination
    let ta = result
        .account(&to)
        .expect("destination token account not found");
    let ta_state = TokenAccount::unpack(&ta.data).expect("failed to unpack token account");
    assert_eq!(
        ta_state.amount, mint_amount,
        "token balance should equal mint amount"
    );

    // Verify mint supply increased
    let mint_acc = result.account(&mint).expect("mint account not found");
    let mint_after = Mint::unpack(&mint_acc.data).expect("failed to unpack mint");
    assert_eq!(
        mint_after.supply, mint_amount,
        "mint supply should equal mint amount"
    );
}

#[test]
fn test_transfer_tokens() {
    let mut svm = setup();

    let authority = Pubkey::new_unique();
    let mint = Pubkey::new_unique();
    let from = Pubkey::new_unique();
    let to = Pubkey::new_unique();

    svm.airdrop(&authority, 10_000_000_000);

    // Pre-create mint
    let mint_state = Mint {
        mint_authority: solana_program_option::COption::Some(authority),
        supply: 1_000_000,
        decimals: 6,
        is_initialized: true,
        freeze_authority: solana_program_option::COption::None,
    };
    svm.set_account(token::create_keyed_mint_account(&mint, &mint_state));

    // Source token account with 1_000_000 tokens
    let from_state = TokenAccount {
        mint,
        owner: authority,
        amount: 1_000_000,
        state: spl_token_interface::state::AccountState::Initialized,
        ..TokenAccount::default()
    };
    svm.set_account(token::create_keyed_token_account(&from, &from_state));

    // Destination token account with 0 tokens
    let to_state = TokenAccount {
        mint,
        owner: Pubkey::new_unique(), // this is the different owner
        amount: 0,
        state: spl_token_interface::state::AccountState::Initialized,
        ..TokenAccount::default()
    };
    svm.set_account(token::create_keyed_token_account(&to, &to_state));

    let transfer_amount: u64 = 500_000;

    let instruction: Instruction = TransferInstruction {
        authority: to_address(&authority),
        from: to_address(&from),
        to: to_address(&to),
        token_program: to_address(&TOKEN_PROGRAM_ID),
        amount: transfer_amount,
    }
    .into();

    let result = svm.process_instruction(
        &instruction,
        &[Account {
            address: authority,
            lamports: 10_000_000_000,
            data: vec![],
            owner: quasar_svm::system_program::ID,
            executable: false,
        }],
    );

    result.assert_success();

    // Verify source balance decreased
    let from_acc = result
        .account(&from)
        .expect("source token account not found");
    let from_after = TokenAccount::unpack(&from_acc.data).expect("failed to unpack source");
    assert_eq!(
        from_after.amount,
        1_000_000 - transfer_amount,
        "source balance should decrease"
    );

    // Verify destination balance increased
    let to_acc = result
        .account(&to)
        .expect("destination token account not found");
    let to_after = TokenAccount::unpack(&to_acc.data).expect("failed to unpack destination");
    assert_eq!(
        to_after.amount, transfer_amount,
        "destination balance should increase"
    );
}
