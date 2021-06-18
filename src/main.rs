//! @brief Main entry poiint for CLI

use {
    crate::utils::txn_utils::ping_instruction,
    clparse::parse_command_line,
    solana_clap_utils::{
        input_parsers::pubkey_of, input_validators::normalize_to_url_if_moniker,
        keypair::DefaultSigner,
    },
    solana_client::rpc_client::RpcClient,
    solana_remote_wallet::remote_wallet::RemoteWalletManager,
    solana_sdk::{commitment_config::CommitmentConfig, native_token::Sol, signature::Signer},
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
                .signer_from_path(&matches, &mut wallet_manager)
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

    use {super::*, solana_validator::test_validator::*};

    #[test]
    fn test_ping() {
        let (test_validator, payer) = TestValidatorGenesis::default().start();
        let (rpc_client, _recent_blockhash, _fee_calculator) = test_validator.rpc_client();

        assert!(matches!(
            ping_instruction(&rpc_client, &payer, CommitmentConfig::confirmed()),
            Ok(_)
        ));
    }
}
