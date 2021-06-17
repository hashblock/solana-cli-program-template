//! solana-cli-program-template Integration Tests (local)
//!
//! Performs local validator test:
//! 1. Assumes solana-test-validator is already started (see note below)
//! 2. Creates/funds wallets and accounts from `keys` directory
//! 3. Tests for sucessful Initialize, Mint, Transfer and Burn of key/value pairs
//! 4. Tests for failing condition handling
//!
//! Note:
//! Running `solana-test-validator` with clean ledger:
//! ```
//! solana-test-validator --bpf-program SampGgdt3wioaoMZhC6LTSbg4pnuvQnSfJpDYeuXQBv ~/solana-cli-program-template/program/target/bpfel-unknown-unknown/release/solana_cli_template_program_bpf.so --ledger ~/solana-cli-program-template/.ledger --reset
//! ```
//! Running `solana-test-validator` with existing ledger and program already loaded
//! ```
//! solana-test-validator --ledger ~/solana-cli-program-template/.ledger
//! ```

pub mod common;

use {
    cli_program_template::prelude::{
        burn_instruction, get_account_for, mint_transaction, transfer_instruction,
        unpack_account_data, Instructions, KEYS_DB, PROG_KEY,
    },
    common::{load_and_initialize_accounts, load_user_wallets, rpc_client_from_config},
    solana_sdk::{instruction::AccountMeta, signer::Signer},
};

#[test]
fn test_initialization_pass() {
    let setup = rpc_client_from_config();
    assert!(setup.is_ok());
    let (rpc_client, funding_keypair) = setup.unwrap();
    assert!(get_account_for(
        &rpc_client,
        &funding_keypair.pubkey(),
        rpc_client.commitment()
    )
    .is_some());
    assert!(get_account_for(&rpc_client, &PROG_KEY.pubkey(), rpc_client.commitment()).is_some());
}

#[test]
fn test_wallet_loading_pass() {
    let (rpc_client, funding_keypair) = rpc_client_from_config().unwrap();
    let loaded_wallets = load_user_wallets(&rpc_client, &funding_keypair, rpc_client.commitment());
    assert_eq!(loaded_wallets.len(), 3);
}

#[test]
fn test_wallet_and_account_initialization_pass() {
    let (rpc_client, funding_keypair) = rpc_client_from_config().unwrap();
    let loaded_wallets = load_user_wallets(&rpc_client, &funding_keypair, rpc_client.commitment());
    assert_eq!(loaded_wallets.len(), 3);
    let initialized_accounts = load_and_initialize_accounts(
        &rpc_client,
        Instructions::InitializeAccount as u8,
        rpc_client.commitment(),
    );
    assert_eq!(initialized_accounts.len(), 3);
    for account in initialized_accounts {
        let (initialized, _) =
            unpack_account_data(&rpc_client, account, rpc_client.commitment()).unwrap();
        assert!(initialized);
    }
}
#[test]
fn test_load_mint_transfer_burn_no_fee_pass() {
    let (rpc_client, funding_keypair) = rpc_client_from_config().unwrap();
    let loaded_wallets = load_user_wallets(&rpc_client, &funding_keypair, rpc_client.commitment());
    assert_eq!(loaded_wallets.len(), 3);
    let initialized_accounts = load_and_initialize_accounts(
        &rpc_client,
        Instructions::InitializeAccount as u8,
        rpc_client.commitment(),
    );
    assert_eq!(initialized_accounts.len(), 3);
    // Setup key/value data and get accounts used in transactions
    let user1 = String::from("User1");
    let user2 = String::from("User2");
    let mint_key = String::from("test_key_1");
    let mint_value = String::from("value for test_key_1");
    let (wallet1, account1) = KEYS_DB.wallet_and_account(user1).unwrap();
    let (wallet2, account2) = KEYS_DB.wallet_and_account(user2).unwrap();

    // Do mint to User1
    let mint_result = mint_transaction(
        &rpc_client,
        &[
            AccountMeta::new(account1.pubkey(), false),
            AccountMeta::new(wallet1.pubkey(), true),
        ],
        wallet1,
        &mint_key,
        &mint_value,
        Instructions::FreeMint as u8,
        rpc_client.commitment(),
    );
    assert!(mint_result.is_ok());
    let (_, btree) = unpack_account_data(&rpc_client, account1, rpc_client.commitment()).unwrap();
    assert!(btree.contains_key(&mint_key));

    // Do transfer of key/value from User1 to User2
    let transfer_result = transfer_instruction(
        &rpc_client,
        &[
            AccountMeta::new(account1.pubkey(), false),
            AccountMeta::new(account2.pubkey(), false),
            AccountMeta::new(wallet1.pubkey(), true),
        ],
        wallet1,
        &mint_key,
        Instructions::FreeTransfer as u8,
        rpc_client.commitment(),
    );
    assert!(transfer_result.is_ok());
    let (_, btree1) = unpack_account_data(&rpc_client, account1, rpc_client.commitment()).unwrap();
    let (_, btree2) = unpack_account_data(&rpc_client, account2, rpc_client.commitment()).unwrap();
    assert!(!btree1.contains_key(&mint_key));
    assert!(btree2.contains_key(&mint_key));
    assert_eq!(btree2.get(&mint_key).unwrap(), &mint_value);

    // Burn the key/value just transfered to User2
    let burn_result = burn_instruction(
        &rpc_client,
        &[
            AccountMeta::new(account2.pubkey(), false),
            AccountMeta::new(wallet2.pubkey(), true),
        ],
        wallet2,
        &mint_key,
        Instructions::FreeBurn as u8,
        rpc_client.commitment(),
    );
    assert!(burn_result.is_ok());
    let (_, btree2) = unpack_account_data(&rpc_client, account2, rpc_client.commitment()).unwrap();
    assert!(!btree2.contains_key(&mint_key));
}

#[test]
fn test_mint_transfer_burn_fail() {
    let (rpc_client, funding_keypair) = rpc_client_from_config().unwrap();
    let loaded_wallets = load_user_wallets(&rpc_client, &funding_keypair, rpc_client.commitment());
    assert_eq!(loaded_wallets.len(), 3);
    let initialized_accounts = load_and_initialize_accounts(
        &rpc_client,
        Instructions::InitializeAccount as u8,
        rpc_client.commitment(),
    );
    assert_eq!(initialized_accounts.len(), 3);
    // Setup key/value data and get accounts used in transactions
    let user1 = String::from("User1");
    let user2 = String::from("User2");
    let mint_key = String::from("test_key_1");
    let mint_value = String::from("value for test_key_1");
    let bad_key = String::from("bad_key_1");
    let (wallet1, account1) = KEYS_DB.wallet_and_account(user1).unwrap();
    let (wallet2, account2) = KEYS_DB.wallet_and_account(user2).unwrap();

    // Fail empty accounts
    let mint_result = mint_transaction(
        &rpc_client,
        &[],
        wallet1,
        &mint_key,
        &mint_value,
        Instructions::FreeMint as u8,
        rpc_client.commitment(),
    );
    assert!(mint_result.is_err());

    // Do mint to User1
    let mint_result = mint_transaction(
        &rpc_client,
        &[
            AccountMeta::new(account1.pubkey(), false),
            AccountMeta::new(wallet1.pubkey(), true),
        ],
        wallet1,
        &mint_key,
        &mint_value,
        Instructions::FreeMint as u8,
        rpc_client.commitment(),
    );
    assert!(mint_result.is_ok());
    let (_, btree) = unpack_account_data(&rpc_client, account1, rpc_client.commitment()).unwrap();
    assert!(btree.contains_key(&mint_key));

    // Attempt to mint something already minted for User1
    let mint_result = mint_transaction(
        &rpc_client,
        &[
            AccountMeta::new(account1.pubkey(), false),
            AccountMeta::new(wallet1.pubkey(), true),
        ],
        wallet1,
        &mint_key,
        &mint_value,
        Instructions::FreeMint as u8,
        rpc_client.commitment(),
    );
    assert!(mint_result.is_err());

    // Attempt to transfer something that does exist
    let transfer_result = transfer_instruction(
        &rpc_client,
        &[
            AccountMeta::new(account1.pubkey(), false),
            AccountMeta::new(account2.pubkey(), false),
            AccountMeta::new(wallet1.pubkey(), true),
        ],
        wallet1,
        &bad_key,
        Instructions::FreeTransfer as u8,
        rpc_client.commitment(),
    );
    assert!(transfer_result.is_err());

    // Attempt to burn something that does not exist
    let burn_result = burn_instruction(
        &rpc_client,
        &[
            AccountMeta::new(account2.pubkey(), false),
            AccountMeta::new(wallet2.pubkey(), true),
        ],
        wallet2,
        &mint_key,
        Instructions::FreeBurn as u8,
        rpc_client.commitment(),
    );
    assert!(burn_result.is_err());
}
