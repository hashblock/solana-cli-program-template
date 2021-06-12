//! @brief Account utilities

use {
    solana_client::rpc_client::RpcClient,
    solana_sdk::{
        account::Account, commitment_config::CommitmentConfig, message::Message, pubkey::Pubkey,
        signature::Keypair, signer::Signer, system_instruction, transaction::Transaction,
    },
};

/// Checks for existence of account
fn account_for_key(
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
) -> Option<Account> {
    account_for_key(rpc_client, &account, commitment_config)
}

/// Fund a wallet by transferring rent-free amount from core account
fn fund_wallet(
    rpc_client: &RpcClient,
    wallet_signer: &dyn Signer,
    signer: &dyn Signer,
    commitment_config: CommitmentConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    let (recent_blockhash, _fee_calculator) = rpc_client
        .get_recent_blockhash()
        .map_err(|err| format!("error: unable to get recent blockhash: {}", err))
        .unwrap();

    let mut transaction = Transaction::new_unsigned(Message::new(
        &[system_instruction::transfer(
            &signer.pubkey(),
            &wallet_signer.pubkey(),
            50_000_000,
        )],
        Some(&signer.pubkey()),
    ));

    transaction
        .try_sign(&vec![signer], recent_blockhash)
        .map_err(|err| format!("error: failed to sign transaction: {}", err))
        .unwrap();
    let _signature = rpc_client
        .send_and_confirm_transaction_with_spinner_and_commitment(&transaction, commitment_config)
        .map_err(|err| format!("error: send transaction: {}", err))
        .unwrap();
    let _account = rpc_client
        .get_account_with_commitment(&wallet_signer.pubkey(), commitment_config)
        .unwrap()
        .value
        .unwrap();
    Ok(())
}

/// Load wallet and, if needed, fund it
pub fn load_wallet(
    rpc_client: &RpcClient,
    wallet_keypair: &Keypair,
    signer: &dyn Signer,
    commitment_config: CommitmentConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    if account_for_key(rpc_client, &wallet_keypair.pubkey(), commitment_config).is_some() {
    } else {
        fund_wallet(rpc_client, wallet_keypair, signer, commitment_config)?;
    };
    Ok(())
}
