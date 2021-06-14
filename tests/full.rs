//! solana-cli-program-template Integration Tests (full)
//!
//! Performs "batteries included" full test:
//! 1. Configured solana-test-validator to load program
//! 2. Creates/funds wallets and accounts from `keys` directory
//! 3. Tests the Initialize, Mint, Transfer and Burn of key/value pairs

mod common;

use {
    cli_program_template::prelude::{
        burn_instruction, get_account_for, mint_transaction, transfer_instruction,
        unpack_account_data, Instructions, KEYS_DB, PROG_KEY,
    },
    common::{clean_ledger_setup_validator, load_and_initialize_accounts, load_user_wallets},
    solana_sdk::{commitment_config::CommitmentConfig, signer::Signer},
};

#[test]
fn test_initialization_pass() {
    let (test_validator, _initial_keypair) = clean_ledger_setup_validator().start();
    let (rpc_client, _, _) = test_validator.rpc_client();
    let cc = CommitmentConfig::confirmed();
    let acc = get_account_for(&rpc_client, &PROG_KEY.pubkey(), cc);
    assert!(acc.is_some());
}

#[test]
fn test_wallet_loading_pass() {
    let (test_validator, funding_keypair) = clean_ledger_setup_validator().start();
    let (rpc_client, _, _) = test_validator.rpc_client();
    let cc = CommitmentConfig::confirmed();
    let loaded_wallets = load_user_wallets(&rpc_client, &funding_keypair, cc);
    assert_eq!(loaded_wallets.len(), 2);
}

#[test]
fn test_wallet_and_account_initialization_pass() {
    let (test_validator, funding_keypair) = clean_ledger_setup_validator().start();
    let (rpc_client, _, _) = test_validator.rpc_client();
    let cc = CommitmentConfig::confirmed();
    let loaded_wallets = load_user_wallets(&rpc_client, &funding_keypair, cc);
    assert_eq!(loaded_wallets.len(), 2);
    let initialized_accounts =
        load_and_initialize_accounts(&rpc_client, Instructions::InitializeAccount as u8, cc);
    assert_eq!(initialized_accounts.len(), 2);
    for account in initialized_accounts {
        let (initialized, _) = unpack_account_data(&rpc_client, account, cc).unwrap();
        assert!(initialized);
    }
}

#[test]
fn test_load_mint_transfer_burn_pass() {
    let (test_validator, funding_keypair) = clean_ledger_setup_validator().start();
    let (rpc_client, _, _) = test_validator.rpc_client();
    let cc = CommitmentConfig::confirmed();
    let loaded_wallets = load_user_wallets(&rpc_client, &funding_keypair, cc);
    assert_eq!(loaded_wallets.len(), 2);
    let initialized_accounts =
        load_and_initialize_accounts(&rpc_client, Instructions::InitializeAccount as u8, cc);
    assert_eq!(initialized_accounts.len(), 2);
    /////////////////////////////////////////////////
    // Instructions that do not charge a service fee
    /////////////////////////////////////////////////
    // Do mint to User1
    let user1 = String::from("User1");
    let user2 = String::from("User2");
    let mint_key = String::from("test_key_1");
    let mint_value = String::from("value for test_key_1");
    let (wallet1, account1) = KEYS_DB.wallet_and_account(user1).unwrap();
    let mint_result = mint_transaction(
        &rpc_client,
        &account1.pubkey(),
        wallet1,
        &mint_key,
        &mint_value,
        Instructions::FreeMint as u8,
        cc,
    );
    assert!(mint_result.is_ok());
    let (_, btree) = unpack_account_data(&rpc_client, account1, cc).unwrap();
    assert!(btree.contains_key(&mint_key));

    // Do transfer of key/value from User1 to User2
    let (wallet2, account2) = KEYS_DB.wallet_and_account(user2).unwrap();
    let transfer_result = transfer_instruction(
        &rpc_client,
        &account1.pubkey(),
        &account2.pubkey(),
        wallet1,
        &mint_key,
        Instructions::FreeTransfer as u8,
        cc,
    );
    assert!(transfer_result.is_ok());
    let (_, btree1) = unpack_account_data(&rpc_client, account1, cc).unwrap();
    let (_, btree2) = unpack_account_data(&rpc_client, account2, cc).unwrap();
    assert!(!btree1.contains_key(&mint_key));
    assert!(btree2.contains_key(&mint_key));
    assert_eq!(btree2.get(&mint_key).unwrap(), &mint_value);

    // Burn the key/value just transfered to User2
    let burn_result = burn_instruction(
        &rpc_client,
        &account2.pubkey(),
        wallet2,
        &mint_key,
        Instructions::FreeBurn as u8,
        cc,
    );
    assert!(burn_result.is_ok());
    let (_, btree2) = unpack_account_data(&rpc_client, account2, cc).unwrap();
    assert!(!btree2.contains_key(&mint_key));
}
