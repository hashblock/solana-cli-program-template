//! solana-cli-program-template Integration Tests (full)
//!
//! Performs "batteries included" full test:
//! 1. Configures solana-test-validator to load program and starts the validator
//! 2. Creates/funds wallets and accounts from `keys` directory
//! 3. Tests for sucessful Initialize, Mint, Transfer and Burn of key/value pairs
//! 4. Tests for failing condition handling

pub mod common;

use {
    cli_program_template::prelude::{
        burn_instruction, get_account_for, mint_transaction, transfer_instruction,
        unpack_account_data, Instructions, KEYS_DB, PROG_KEY,
    },
    common::{clean_ledger_setup_validator, load_and_initialize_accounts, load_user_wallets},
    solana_sdk::{commitment_config::CommitmentConfig, instruction::AccountMeta, signer::Signer},
};

#[test]
fn test_initialization_pass() {
    let (test_validator, _initial_keypair) = clean_ledger_setup_validator().start();
    let rpc_client = test_validator.get_rpc_client();
    let cc = CommitmentConfig::confirmed();
    let acc = get_account_for(&rpc_client, &PROG_KEY.pubkey(), cc);
    assert!(acc.is_some());
}

#[test]
fn test_wallet_loading_pass() {
    let (test_validator, funding_keypair) = clean_ledger_setup_validator().start();
    let rpc_client = test_validator.get_rpc_client();
    let cc = CommitmentConfig::confirmed();
    let loaded_wallets = load_user_wallets(&rpc_client, &funding_keypair, cc);
    assert_eq!(loaded_wallets.len(), 3);
}

#[test]
fn test_wallet_and_account_initialization_pass() {
    let (test_validator, funding_keypair) = clean_ledger_setup_validator().start();
    let rpc_client = test_validator.get_rpc_client();
    let cc = CommitmentConfig::confirmed();
    let loaded_wallets = load_user_wallets(&rpc_client, &funding_keypair, cc);
    assert_eq!(loaded_wallets.len(), 3);
    let initialized_accounts =
        load_and_initialize_accounts(&rpc_client, Instructions::InitializeAccount as u8, cc);
    assert_eq!(initialized_accounts.len(), 3);
    for account in initialized_accounts {
        let (initialized, _) = unpack_account_data(&rpc_client, account, cc).unwrap();
        assert!(initialized);
    }
}

#[test]
fn test_load_mint_transfer_burn_no_fee_pass() {
    let (test_validator, funding_keypair) = clean_ledger_setup_validator().start();
    let rpc_client = test_validator.get_rpc_client();
    let cc = CommitmentConfig::confirmed();
    let loaded_wallets = load_user_wallets(&rpc_client, &funding_keypair, cc);
    assert_eq!(loaded_wallets.len(), 3);
    let initialized_accounts =
        load_and_initialize_accounts(&rpc_client, Instructions::InitializeAccount as u8, cc);
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
        cc,
    );
    assert!(mint_result.is_ok());
    let (_, btree) = unpack_account_data(&rpc_client, account1, cc).unwrap();
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
        &[
            AccountMeta::new(account2.pubkey(), false),
            AccountMeta::new(wallet2.pubkey(), true),
        ],
        wallet2,
        &mint_key,
        Instructions::FreeBurn as u8,
        cc,
    );
    assert!(burn_result.is_ok());
    let (_, btree2) = unpack_account_data(&rpc_client, account2, cc).unwrap();
    assert!(!btree2.contains_key(&mint_key));
}

#[test]
fn test_load_mint_transfer_burn_with_fee_pass() {
    let (test_validator, funding_keypair) = clean_ledger_setup_validator().start();
    let rpc_client = test_validator.get_rpc_client();
    let cc = CommitmentConfig::confirmed();
    let loaded_wallets = load_user_wallets(&rpc_client, &funding_keypair, cc);
    assert_eq!(loaded_wallets.len(), 3);
    let initialized_accounts =
        load_and_initialize_accounts(&rpc_client, Instructions::InitializeAccount as u8, cc);
    assert_eq!(initialized_accounts.len(), 3);
    // Setup key/value data and get accounts used in transactions
    let user1 = String::from("User1");
    let user2 = String::from("User2");
    let service = String::from("Service");
    let mint_key = String::from("test_key_1");
    let mint_value = String::from("value for test_key_1");
    let (wallet1, account1) = KEYS_DB.wallet_and_account(user1).unwrap();
    let (wallet2, account2) = KEYS_DB.wallet_and_account(user2).unwrap();
    let (_serivce_wallet, service_account) = KEYS_DB.wallet_and_account(service).unwrap();
    // Mint with fee
    let service_pre_tx_lamport = get_account_for(&rpc_client, &service_account.pubkey(), cc)
        .unwrap()
        .lamports;
    let mint_result = mint_transaction(
        &rpc_client,
        &[
            AccountMeta::new(account1.pubkey(), false),
            AccountMeta::new(service_account.pubkey(), false),
            AccountMeta::new(wallet1.pubkey(), true),
        ],
        wallet1,
        &mint_key,
        &mint_value,
        Instructions::MintWithFee as u8,
        cc,
    );
    assert!(mint_result.is_ok());
    let (_, btree) = unpack_account_data(&rpc_client, account1, cc).unwrap();
    assert!(btree.contains_key(&mint_key));
    let service_credited = get_account_for(&rpc_client, &service_account.pubkey(), cc)
        .unwrap()
        .lamports
        - service_pre_tx_lamport;
    assert!(service_credited == 10);

    // Do transfer of key/value from User1 to User2 with fee
    let service_pre_tx_lamport = get_account_for(&rpc_client, &service_account.pubkey(), cc)
        .unwrap()
        .lamports;
    let transfer_result = transfer_instruction(
        &rpc_client,
        &[
            AccountMeta::new(account1.pubkey(), false),
            AccountMeta::new(account2.pubkey(), false),
            AccountMeta::new(service_account.pubkey(), false),
            AccountMeta::new(wallet1.pubkey(), true),
        ],
        wallet1,
        &mint_key,
        Instructions::TransferWithFee as u8,
        cc,
    );
    assert!(transfer_result.is_ok());
    let (_, btree1) = unpack_account_data(&rpc_client, account1, cc).unwrap();
    let (_, btree2) = unpack_account_data(&rpc_client, account2, cc).unwrap();
    assert!(!btree1.contains_key(&mint_key));
    assert!(btree2.contains_key(&mint_key));
    assert_eq!(btree2.get(&mint_key).unwrap(), &mint_value);
    let service_credited = get_account_for(&rpc_client, &service_account.pubkey(), cc)
        .unwrap()
        .lamports
        - service_pre_tx_lamport;
    assert!(service_credited == 40);

    // Burn the key/value just transfered to User2 with fee
    let service_pre_tx_lamport = get_account_for(&rpc_client, &service_account.pubkey(), cc)
        .unwrap()
        .lamports;
    let burn_result = burn_instruction(
        &rpc_client,
        &[
            AccountMeta::new(account2.pubkey(), false),
            AccountMeta::new(service_account.pubkey(), false),
            AccountMeta::new(wallet2.pubkey(), true),
        ],
        wallet2,
        &mint_key,
        Instructions::BurnWithFee as u8,
        cc,
    );
    assert!(burn_result.is_ok());
    let (_, btree2) = unpack_account_data(&rpc_client, account2, cc).unwrap();
    assert!(!btree2.contains_key(&mint_key));
    let service_credited = get_account_for(&rpc_client, &service_account.pubkey(), cc)
        .unwrap()
        .lamports
        - service_pre_tx_lamport;
    assert!(service_credited == 15);
}

#[test]
fn test_mint_transfer_burn_fail() {
    let (test_validator, funding_keypair) = clean_ledger_setup_validator().start();
    let rpc_client = test_validator.get_rpc_client();
    let cc = CommitmentConfig::confirmed();
    let loaded_wallets = load_user_wallets(&rpc_client, &funding_keypair, cc);
    assert_eq!(loaded_wallets.len(), 3);
    let initialized_accounts =
        load_and_initialize_accounts(&rpc_client, Instructions::InitializeAccount as u8, cc);
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
        cc,
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
        cc,
    );
    assert!(mint_result.is_ok());
    let (_, btree) = unpack_account_data(&rpc_client, account1, cc).unwrap();
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
        cc,
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
        cc,
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
        cc,
    );
    assert!(burn_result.is_err());
}
