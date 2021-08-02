//! @brief command line setup and parse

use {
    clap::{
        crate_description, crate_name, crate_version, App, AppSettings, Arg, ArgMatches, SubCommand,
    },
    solana_clap_utils::input_validators::{is_url_or_moniker, is_valid_pubkey, is_valid_signer},
};

/// Construct the cli input model and parse command line
pub fn parse_command_line() -> ArgMatches<'static> {
    App::new(crate_name!())
        .about(crate_description!())
        .version(crate_version!())
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg({
            let arg = Arg::with_name("config_file")
                .short("C")
                .long("config")
                .value_name("PATH")
                .takes_value(true)
                .global(true)
                .help("Configuration file to use");
            if let Some(ref config_file) = *solana_cli_config::CONFIG_FILE {
                arg.default_value(config_file)
            } else {
                arg
            }
        })
        .arg(
            Arg::with_name("keypair")
                .long("keypair")
                .value_name("KEYPAIR")
                .validator(is_valid_signer)
                .takes_value(true)
                .global(true)
                .help("Filepath or URL to a keypair [default: client keypair]"),
        )
        .arg(
            Arg::with_name("verbose")
                .long("verbose")
                .short("v")
                .takes_value(false)
                .global(true)
                .help("Show additional information"),
        )
        .arg(
            Arg::with_name("json_rpc_url")
                .short("u")
                .long("url")
                .value_name("URL")
                .takes_value(true)
                .global(true)
                .validator(is_url_or_moniker)
                .help("JSON RPC URL for the cluster [default: value from configuration file]"),
        )
        .subcommand(
            SubCommand::with_name("balance").about("Get balance").arg(
                Arg::with_name("address")
                    .validator(is_valid_pubkey)
                    .value_name("ADDRESS")
                    .takes_value(true)
                    .index(1)
                    .help("Address to get the balance of"),
            ),
        )
        .subcommand(
            SubCommand::with_name("mint")
                .about("Mint a new key/value pair to an account")
                .arg(
                    Arg::with_name("to-owner")
                        .display_order(1)
                        .long("to-owner")
                        .short("t")
                        .required(true)
                        .takes_value(true)
                        .help("Owner of accounts")
                        .possible_values(&["User1", "User2"]),
                )
                .arg(
                    Arg::with_name("key")
                        .display_order(2)
                        .long("key")
                        .short("k")
                        .required(true)
                        .takes_value(true)
                        .help("The key of key/value pair"),
                )
                .arg(
                    Arg::with_name("value")
                        .display_order(3)
                        .long("value")
                        .required(true)
                        .min_values(1)
                        .help("The value string of key/value pair"),
                ),
        )
        .subcommand(
            SubCommand::with_name("transfer")
                .about("Transfer a key/value pair from one account to another")
                .arg(
                    Arg::with_name("from-owner")
                        .display_order(1)
                        .long("from-owner")
                        .short("f")
                        .required(true)
                        .takes_value(true)
                        .help("Owner to transfer from")
                        .possible_values(&["User1", "User2"]),
                )
                .arg(
                    Arg::with_name("to-owner")
                        .display_order(2)
                        .long("to-owner")
                        .short("t")
                        .required(true)
                        .takes_value(true)
                        .help("Owner to transfer to")
                        .possible_values(&["User1", "User2"]),
                )
                .arg(
                    Arg::with_name("key")
                        .display_order(3)
                        .long("key")
                        .short("k")
                        .required(true)
                        .takes_value(true)
                        .help("The key of key/value pair to transfer"),
                ),
        )
        .subcommand(
            SubCommand::with_name("burn")
                .about("Burn (delete) a key/value pair from an account")
                .arg(
                    Arg::with_name("from-owner")
                        .display_order(1)
                        .long("from-owner")
                        .short("f")
                        .required(true)
                        .takes_value(true)
                        .help("Owner to burn key/value from")
                        .possible_values(&["User1", "User2"]),
                )
                .arg(
                    Arg::with_name("key")
                        .display_order(2)
                        .long("key")
                        .short("k")
                        .required(true)
                        .takes_value(true)
                        .help("The key of key/value pair to burn"),
                ),
        )
        .subcommand(SubCommand::with_name("ping").about("Send a ping transaction"))
        .get_matches()
}
