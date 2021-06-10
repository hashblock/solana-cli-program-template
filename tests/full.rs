//! solana-cli-program-template Integration Tests (full)
//!
//! Performs "batteries included" full test:
//! 1. Configured solana-test-validator to load program
//! 2. Creates/funds wallets and accounts from `keys` directory
//! 3. Tests the Initialize, Mint, Transfer and Burn of key/value pairs

use {
    cli_template::prelude::{get_account_for, PROG_KEY},
    std::{path::PathBuf, str::FromStr},
};

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{commitment_config::CommitmentConfig, signer::Signer};
    // use solana_client::rpc_cache::test;
    use solana_validator::test_validator::*;
    const LEDGER_PATH: &str = "./.ledger";
    const PROG_PATH: &str = "program/target/bpfel-unknown-unknown/release/";
    const PROG_NAME: &str = "solana_cli_template_program_bpf";

    /// Setup the test validator properties
    fn setup_validator() -> TestValidatorGenesis {
        std::env::set_var("BPF_OUT_DIR", PROG_PATH);
        let mut test_validator = TestValidatorGenesis::default();
        test_validator.ledger_path(LEDGER_PATH);
        test_validator.add_program(PROG_NAME, PROG_KEY.pubkey());
        test_validator
    }

    /// Cleans up existing ledger before running
    fn clean_ledger_setup_validator() -> TestValidatorGenesis {
        if PathBuf::from_str(LEDGER_PATH).unwrap().exists() {
            std::fs::remove_dir_all(LEDGER_PATH).unwrap();
        }
        setup_validator()
    }

    #[test]
    fn test_initialization() {
        let (test_validator, _initial_keypair) = clean_ledger_setup_validator().start();
        let (rpc_client, _, _) = test_validator.rpc_client();
        let cc = CommitmentConfig::confirmed();
        let acc = get_account_for(&rpc_client, &PROG_KEY.pubkey(), cc);
        assert!(acc.is_ok());
    }
}
