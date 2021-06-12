//! @brief Common function for testing

use {
    cli_template::prelude::{get_account_for, load_wallet, KEYS_DB, PROG_KEY},
    solana_client::rpc_client::RpcClient,
    solana_sdk::{commitment_config::CommitmentConfig, signature::Keypair, signer::Signer},
    solana_validator::test_validator::TestValidatorGenesis,
    std::{path::PathBuf, str::FromStr},
};

const LEDGER_PATH: &str = "./.ledger";
const PROG_PATH: &str = "program/target/bpfel-unknown-unknown/release/";
const PROG_NAME: &str = "solana_cli_template_program_bpf";

/// Setup the test validator properties
pub fn setup_validator() -> TestValidatorGenesis {
    std::env::set_var("BPF_OUT_DIR", PROG_PATH);
    let mut test_validator = TestValidatorGenesis::default();
    test_validator.ledger_path(LEDGER_PATH);
    test_validator.add_program(PROG_NAME, PROG_KEY.pubkey());
    test_validator
}

/// Cleans up existing ledger before running
pub fn clean_ledger_setup_validator() -> TestValidatorGenesis {
    if PathBuf::from_str(LEDGER_PATH).unwrap().exists() {
        std::fs::remove_dir_all(LEDGER_PATH).unwrap();
    }
    setup_validator()
}

/// Batch load all user wallets
pub fn load_user_wallets<'a>(
    rpc_client: &RpcClient,
    funding_source: &dyn Signer,
    commitment_config: CommitmentConfig,
) -> Vec<&'a Keypair> {
    let mut wallets = Vec::<&Keypair>::new();
    for holder in KEYS_DB.key_owners() {
        let (wallet, _account) = KEYS_DB.wallet_and_account(holder.clone()).unwrap();
        if let Some(_account) = get_account_for(rpc_client, &wallet.pubkey(), commitment_config) {
            panic!()
        }
        let result = load_wallet(&rpc_client, wallet, funding_source, commitment_config);
        assert!(result.is_ok());
        wallets.push(wallet);
    }
    wallets
}
