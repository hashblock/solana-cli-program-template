//! @brief Main entry poiint for CLI

use {
    cli_program_template::prelude::{
        burn_instruction, load_account, load_wallet, mint_transaction, ping_instruction,
        transfer_instruction, unpack_account_data, Instructions, KEYS_DB, PROG_KEY,
    },
    clparse::parse_command_line,
    sol_template_shared::ACCOUNT_STATE_SPACE,
    solana_clap_utils::{
        input_parsers::pubkey_of, input_validators::normalize_to_url_if_moniker,
        keypair::DefaultSigner,
    },
    solana_client::rpc_client::RpcClient,
    solana_remote_wallet::remote_wallet::RemoteWalletManager,
    solana_sdk::{
        commitment_config::CommitmentConfig,
        instruction::AccountMeta,
        native_token::Sol,
        signature::{Keypair, Signer},
    },
    std::{process::exit, sync::Arc},
};
pub mod clparse;
pub mod utils;

struct Config {
    commitment_config: CommitmentConfig,
    default_signer: Box<dyn Signer>,
    json_rpc_url: String,
    verbose: bool,
}

/// Wallet and account verification and load
///
/// Will search KEYS_DB for existence of the owner string and return the wallet and account keys
/// and, optionally, fund the wallet and create and initialize the account if needed
///
/// # Example
/// ```ignore
/// validate_user_account_and_load(&rpc_client, funding_source, commitment_config, "User1")?;
/// ```
fn validate_user_accounts_and_load<'a>(
    rpc_client: &RpcClient,
    funding_source: &dyn Signer,
    commitment_config: CommitmentConfig,
    owner: &str,
) -> Result<(&'a Keypair, &'a Keypair), Box<dyn std::error::Error>> {
    // Check in KEYS_DB for owner
    let (wallet, account) = KEYS_DB.wallet_and_account(owner.to_string())?;
    // Fund wallet if required
    load_wallet(rpc_client, wallet, funding_source, commitment_config)?;
    // Create and initialize account if required
    load_account(
        rpc_client,
        account,
        wallet,
        &PROG_KEY.pubkey(),
        ACCOUNT_STATE_SPACE as u64,
        Instructions::InitializeAccount as u8,
        commitment_config,
    )?;
    Ok((wallet, account))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_matches = parse_command_line();
    let (sub_command, sub_matches) = app_matches.subcommand();
    let matches = sub_matches.unwrap();
    let mut wallet_manager: Option<Arc<RemoteWalletManager>> = None;

    let config = {
        let cli_config = if let Some(config_file) = matches.value_of("config_file") {
            solana_cli_config::Config::load(config_file).unwrap_or_default()
        } else {
            solana_cli_config::Config::default()
        };

        let default_signer = DefaultSigner::new(
            "keypair".to_string(),
            matches
                .value_of(&"keypair")
                .map(|s| s.to_string())
                .unwrap_or_else(|| cli_config.keypair_path.clone()),
        );

        Config {
            json_rpc_url: normalize_to_url_if_moniker(
                matches
                    .value_of("json_rpc_url")
                    .unwrap_or(&cli_config.json_rpc_url)
                    .to_string(),
            ),
            default_signer: default_signer
                .signer_from_path(matches, &mut wallet_manager)
                .unwrap_or_else(|err| {
                    eprintln!("error: {}", err);
                    exit(1);
                }),
            verbose: matches.is_present("verbose"),
            commitment_config: CommitmentConfig::confirmed(),
        }
    };
    solana_logger::setup_with_default("solana=info");

    if config.verbose {
        println!("JSON RPC URL: {}", config.json_rpc_url);
    }
    let rpc_client = RpcClient::new(config.json_rpc_url.clone());
    // Load the keys_db

    match (sub_command, sub_matches) {
        ("balance", Some(arg_matches)) => {
            let address =
                pubkey_of(arg_matches, "address").unwrap_or_else(|| config.default_signer.pubkey());
            println!(
                "{} has a balance of {}",
                address,
                Sol(rpc_client
                    .get_balance_with_commitment(&address, config.commitment_config)?
                    .value)
            );
        }
        ("mint", Some(_arg_matchs)) => {
            let owner = matches.value_of("to-owner").unwrap();
            let key = matches.value_of("key").unwrap();
            let value = {
                let value: Vec<_> = matches.values_of("value").unwrap().collect();
                value.join(" ")
            };
            // Verify the owner is a valid account
            let (wallet, account) = validate_user_accounts_and_load(
                &rpc_client,
                config.default_signer.as_ref(),
                config.commitment_config,
                owner,
            )?;
            // Execute command
            mint_transaction(
                &rpc_client,
                &[
                    AccountMeta::new(account.pubkey(), false),
                    AccountMeta::new(wallet.pubkey(), true),
                ],
                wallet,
                key,
                &value,
                Instructions::FreeMint as u8,
                config.commitment_config,
            )?;
            let (_, btree) = unpack_account_data(&rpc_client, account, config.commitment_config)?;
            println!("{} to account key/value store {:?}", owner, btree);
        }
        ("transfer", Some(_arg_matchs)) => {
            let from_owner = matches.value_of("from-owner").unwrap();
            let to_owner = matches.value_of("to-owner").unwrap();
            let key = matches.value_of("key").unwrap();
            // Verify that from and to owners are different and both are
            // valid
            let (from_wallet, from_account) = validate_user_accounts_and_load(
                &rpc_client,
                config.default_signer.as_ref(),
                config.commitment_config,
                from_owner,
            )?;
            let (_, to_account) = validate_user_accounts_and_load(
                &rpc_client,
                config.default_signer.as_ref(),
                config.commitment_config,
                to_owner,
            )?;
            // Execute command
            transfer_instruction(
                &rpc_client,
                &[
                    AccountMeta::new(from_account.pubkey(), false),
                    AccountMeta::new(to_account.pubkey(), false),
                    AccountMeta::new(from_wallet.pubkey(), true),
                ],
                from_wallet,
                key,
                Instructions::FreeTransfer as u8,
                config.commitment_config,
            )?;
            let (_, btree) =
                unpack_account_data(&rpc_client, from_account, config.commitment_config)?;
            println!("{} from account key/value store {:?}", from_owner, btree);
            let (_, btree) =
                unpack_account_data(&rpc_client, to_account, config.commitment_config)?;
            println!("{} to account key/value store {:?}", to_owner, btree);
        }
        ("burn", Some(_arg_matchs)) => {
            let owner = matches.value_of("from-owner").unwrap();
            let key = matches.value_of("key").unwrap();
            // Verify the owner is a valid account
            let (wallet, account) = validate_user_accounts_and_load(
                &rpc_client,
                config.default_signer.as_ref(),
                config.commitment_config,
                owner,
            )?;
            // Execute command
            burn_instruction(
                &rpc_client,
                &[
                    AccountMeta::new(account.pubkey(), false),
                    AccountMeta::new(wallet.pubkey(), true),
                ],
                wallet,
                key,
                Instructions::FreeBurn as u8,
                config.commitment_config,
            )?;
            let (_, btree) = unpack_account_data(&rpc_client, account, config.commitment_config)?;
            println!("{} from account key/value store {:?}", owner, btree);
        }
        ("ping", Some(_arg_matches)) => {
            let signature = ping_instruction(
                &rpc_client,
                config.default_signer.as_ref(),
                config.commitment_config,
            )
            .unwrap_or_else(|err| {
                eprintln!("error: send transaction: {}", err);
                exit(1);
            });
            println!("Signature: {}", signature);
        }
        _ => unreachable!(),
    };

    Ok(())
}

#[cfg(test)]
mod test {
    use borsh::{BorshDeserialize, BorshSerialize};
    use solana_sdk::pubkey::Pubkey;

    use {super::*, solana_validator::test_validator::*};

    #[test]
    fn test_ping() {
        let (test_validator, payer) = TestValidatorGenesis::default().start();
        let rpc_client = test_validator.get_rpc_client();

        assert!(matches!(
            ping_instruction(&rpc_client, &payer, CommitmentConfig::confirmed()),
            Ok(_)
        ));
    }

    #[test]
    fn test_borsh() {
        #[repr(C)]
        #[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
        pub struct UpdateMetadataAccountArgs {
            pub data: Option<String>,
            pub update_authority: Option<Pubkey>,
            pub primary_sale_happened: Option<bool>,
        }
        let faux = UpdateMetadataAccountArgs {
            data: Some(String::from("This")),
            update_authority: Some(Pubkey::default()),
            primary_sale_happened: Some(true),
        };
        let bout = faux.try_to_vec().unwrap();
        println!("{:?}", bout);
        let in_faux = UpdateMetadataAccountArgs::try_from_slice(&bout).unwrap();
        println!("{:?}", in_faux);
    }
}
