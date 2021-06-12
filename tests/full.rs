//! solana-cli-program-template Integration Tests (full)
//!
//! Performs "batteries included" full test:
//! 1. Configured solana-test-validator to load program
//! 2. Creates/funds wallets and accounts from `keys` directory
//! 3. Tests the Initialize, Mint, Transfer and Burn of key/value pairs

mod common;

use {
    cli_template::prelude::{get_account_for, unpack_account_data, PROG_KEY},
    common::{clean_ledger_setup_validator, load_and_initialize_accounts, load_user_wallets},
    solana_sdk::{commitment_config::CommitmentConfig, signer::Signer},
};

#[test]
fn test_initialization() {
    let (test_validator, _initial_keypair) = clean_ledger_setup_validator().start();
    let (rpc_client, _, _) = test_validator.rpc_client();
    let cc = CommitmentConfig::confirmed();
    let acc = get_account_for(&rpc_client, &PROG_KEY.pubkey(), cc);
    assert!(acc.is_some());
}

#[test]
fn test_wallet_loading() {
    let (test_validator, funding_keypair) = clean_ledger_setup_validator().start();
    let (rpc_client, _, _) = test_validator.rpc_client();
    let cc = CommitmentConfig::confirmed();
    let loaded_wallets = load_user_wallets(&rpc_client, &funding_keypair, cc);
    assert_eq!(loaded_wallets.len(), 2);
}

#[test]
fn test_wallet_and_account_initialization() {
    let (test_validator, funding_keypair) = clean_ledger_setup_validator().start();
    // solana_logger::setup_with_default("solana=info");
    let (rpc_client, _, _) = test_validator.rpc_client();
    let cc = CommitmentConfig::confirmed();
    let loaded_wallets = load_user_wallets(&rpc_client, &funding_keypair, cc);
    assert_eq!(loaded_wallets.len(), 2);
    let initialized_accounts = load_and_initialize_accounts(&rpc_client, cc);
    assert_eq!(initialized_accounts.len(), 2);
    for account in initialized_accounts {
        let (initialized, _) = unpack_account_data(&rpc_client, account, cc).unwrap();
        assert!(initialized);
    }
}
