//! @brief Account utilities

use {
    solana_client::rpc_client::RpcClient,
    solana_sdk::{account::Account, commitment_config::CommitmentConfig, pubkey::Pubkey},
    std::error::Error,
};

/// Checks for existence of account
fn check_for_account(
    rpc_client: &RpcClient,
    key: &Pubkey,
    commitment_config: CommitmentConfig,
) -> Option<Account> {
    rpc_client
        .get_account_with_commitment(key, commitment_config)
        .unwrap()
        .value
}

/// Gets the account from the ledger
pub fn get_account_for(
    rpc_client: &RpcClient,
    account: &Pubkey,
    commitment_config: CommitmentConfig,
) -> Result<Account, Box<dyn Error>> {
    Ok(check_for_account(rpc_client, &account, commitment_config).unwrap())
}
