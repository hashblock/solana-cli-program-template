//! @brief Account utilities

use {
    crate::utils::keys_db::PROG_KEY,
    borsh::*,
    solana_client::rpc_client::RpcClient,
    solana_sdk::{
        account::Account,
        commitment_config::CommitmentConfig,
        instruction::{AccountMeta, Instruction},
        message::Message,
        pubkey::Pubkey,
        signature::Keypair,
        signer::Signer,
        system_instruction,
        transaction::Transaction,
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

/// Create a new program account with account state data allocation
fn new_account(
    rpc_client: &RpcClient,
    wallet_signer: &dyn Signer,
    account_pair: &dyn Signer,
    program_owner: &Pubkey,
    state_space: u64,
    initialize_instruction_id: u8,
    commitment_config: CommitmentConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    let account_lamports = rpc_client
        .get_minimum_balance_for_rent_exemption(state_space as usize)
        .unwrap();

    let instruction_data = vec![vec![initialize_instruction_id]];

    let mut transaction = Transaction::new_with_payer(
        &[
            system_instruction::create_account(
                &wallet_signer.pubkey(),
                &account_pair.pubkey(),
                account_lamports,
                state_space,
                program_owner,
            ),
            Instruction::new_with_borsh(
                *program_owner,
                &instruction_data,
                vec![
                    AccountMeta::new(account_pair.pubkey(), false),
                    AccountMeta::new(wallet_signer.pubkey(), false),
                ],
            ),
        ],
        Some(&wallet_signer.pubkey()),
    );

    let (recent_blockhash, _fee_calculator) = rpc_client
        .get_recent_blockhash()
        .map_err(|err| format!("error: unable to get recent blockhash: {}", err))
        .unwrap();
    transaction
        .try_sign(&vec![wallet_signer, account_pair], recent_blockhash)
        .map_err(|err| format!("error: failed to sign transaction: {}", err))
        .unwrap();
    let _signature = rpc_client
        .send_and_confirm_transaction_with_spinner_and_commitment(&transaction, commitment_config)
        .map_err(|err| format!("error: send transaction: {}", err))
        .unwrap();
    let _account = rpc_client
        .get_account_with_commitment(&account_pair.pubkey(), commitment_config)
        .map_err(|err| format!("error: getting account after initialization: {}", err))
        .unwrap();

    Ok(())
}

/// Load account with size
pub fn load_account(
    rpc_client: &RpcClient,
    account_pair: &Keypair,
    wallet_signer: &dyn Signer,
    program_owner: &Pubkey,
    space: u64,
    initialize_instruction: u8,
    commitment_config: CommitmentConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    match get_account_for(rpc_client, &account_pair.pubkey(), commitment_config) {
        Some(_) => {}
        None => new_account(
            rpc_client,
            wallet_signer,
            account_pair,
            program_owner,
            space,
            initialize_instruction,
            commitment_config,
        )
        .unwrap(),
    };
    Ok(())
}

/// Submits the program instruction as per the
/// instruction definition
pub fn submit_transaction(
    rpc_client: &RpcClient,
    wallet_signer: &dyn Signer,
    instruction: Instruction,
    commitment_config: CommitmentConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut transaction =
        Transaction::new_unsigned(Message::new(&[instruction], Some(&wallet_signer.pubkey())));
    let (recent_blockhash, _fee_calculator) = rpc_client
        .get_recent_blockhash()
        .map_err(|err| format!("error: unable to get recent blockhash: {}", err))
        .unwrap();
    transaction
        .try_sign(&vec![wallet_signer], recent_blockhash)
        .map_err(|err| format!("error: failed to sign transaction: {}", err))
        .unwrap();
    let _signature = rpc_client
        .send_and_confirm_transaction_with_spinner_and_commitment(&transaction, commitment_config)
        .map_err(|err| format!("error: send transaction: {}", err));
    Ok(())
}

/// Perform a mint transaction consisting of a key/value pair
pub fn mint_transaction(
    rpc_client: &RpcClient,
    account: &Pubkey,
    wallet_signer: &dyn Signer,
    mint_key: &str,
    mint_value: &str,
    mint_instruction_id: u8,
    commitment_config: CommitmentConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![
        vec![mint_instruction_id],
        mint_key.try_to_vec().unwrap(),
        mint_value.try_to_vec().unwrap(),
    ];
    let instruction = Instruction::new_with_borsh(
        PROG_KEY.pubkey(),
        &data,
        vec![
            AccountMeta::new(*account, false),
            AccountMeta::new(wallet_signer.pubkey(), true),
        ],
    );
    submit_transaction(rpc_client, wallet_signer, instruction, commitment_config)?;
    Ok(())
}

/// Transfer a minted key/value from one account to another account
pub fn transfer_instruction(
    rpc_client: &RpcClient,
    account_from: &Pubkey,
    account_to: &Pubkey,
    wallet_signer: &dyn Signer,
    transfer_key: &str,
    transfer_instruction_id: u8,
    commitment_config: CommitmentConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![
        vec![transfer_instruction_id],
        transfer_key.try_to_vec().unwrap(),
    ];
    let instruction = Instruction::new_with_borsh(
        PROG_KEY.pubkey(),
        &data,
        vec![
            AccountMeta::new(*account_from, false),
            AccountMeta::new(*account_to, false),
            AccountMeta::new(wallet_signer.pubkey(), true),
        ],
    );
    submit_transaction(rpc_client, wallet_signer, instruction, commitment_config)?;
    Ok(())
}

/// Burn, delete, the key/value from the owning account
pub fn burn_instruction(
    rpc_client: &RpcClient,
    account_from: &Pubkey,
    wallet_signer: &dyn Signer,
    burn_key: &str,
    burn_instruction_id: u8,
    commitment_config: CommitmentConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![vec![burn_instruction_id], burn_key.try_to_vec().unwrap()];
    let instruction = Instruction::new_with_borsh(
        PROG_KEY.pubkey(),
        &data,
        vec![
            AccountMeta::new(*account_from, false),
            AccountMeta::new(wallet_signer.pubkey(), true),
        ],
    );
    submit_transaction(rpc_client, wallet_signer, instruction, commitment_config)?;
    Ok(())
}
