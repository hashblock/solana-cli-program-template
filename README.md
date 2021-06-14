## Quick start
1. Install Rust from https://rustup.rs/
2. `cargo run`

## About
`solana-cli-program template` is a sample app demonstrating the creation of a minimal CLI application written in Rust to interact with Solana.
It provides three pieces of functionality:

- `ping`: Creates a transaction sending 0 SOL from the signer's account to the signer's account. Returns the signature of the transaction.
- `balance`: Returns the account's balance.
- `help`: Tips for using the app. This is an off-chain operation.

## Running locally step-by-step
1. Prepare account:
  - Start a local node: run `solana-test-validator`.
  - Generate a keypair: `solana-keygen new -o test.json`.
  - Add 100 SOL to the corresponding account `solana airdrop --url http://127.0.0.1:8899 --keypair test.json 100`.

2. Build app: `cargo run`.

3. Ping:
  ```
  $ cargo run -- ping --url http://127.0.0.1:8899 --keypair test.json
  Signature: 2Y863JX96RTqbeGfcvQowVt1V91Dgs2LZfVgQ3mGJPmYu24sUTYmfkArHAAgj4uFqP75bm9GXU9DYjiMFxahQJUC
  ```

4. Balance:
  ```
  $ cargo run -- balance --url http://127.0.0.1:8899 --keypair test.json
  3dSRGE3wYCcGWFrxAsQs5PaBqtJzzxdTzY2ypXNFUji9 has a balance of ◎99.999995000 // balance less than 100 because of ping operation above
  ```

5. Run help for the complete list of options:
  ```
  $ cargo run -- --help
  cli-program-template 0.1.0


  USAGE:
      cli-program-template [FLAGS] [OPTIONS] <SUBCOMMAND>

  FLAGS:
      -h, --help       Prints help information
      -V, --version    Prints version information
      -v, --verbose    Show additional information

  OPTIONS:
      -C, --config <PATH>        Configuration file to use [default: /Users/user/.config/solana/cli/config.yml]
          --url <URL>            JSON RPC URL for the cluster [default: value from configuration file]
          --keypair <KEYPAIR>    Filepath or URL to a keypair [default: client keypair]

  SUBCOMMANDS:
      balance    Get balance
      help       Prints this message or the help of the given subcommand(s)
      ping       Send a ping transaction
  ```