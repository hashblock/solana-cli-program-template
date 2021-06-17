# Integration testing

This directory contains integration tests of the sample program (`solana-cli-program-template/program`).

The idea is to offer modularized integration test scanarios found in `tests/`.

## Layout
* Keys: The program and a couple of account keys, generated with `solana-keygen`. They are located in `keys/`.
If you want to add more keys, follow the examples given and don't forget to update `keys/keys_db.yml` file.
* Program: The sample program is in `program/`. Prior to running the tests make sure you:
```
cd program
cargo build-bpf
cd ..
```
## Scenarios
The following describe the individual modules and intent:
1. `tests/full.rs` (a.k.a. no-batteries-required) - Integration tests that automatically loads `solana-test-validator`, the sample programs and any external keys. Each tests included in `full.rs`, in effect, run with a new clean *ledger* each time a test is run. Includes both positive and negative testing.
2. `tests/thin.rs` (a.k.a. Local) - Assumes that you've either started the `solana-test-validator` locally or your configuration is pointing to one of the solana networks which have the program already deployed. The same tests that are run in `tests/full.rs` are run here.
For example; To test locally with the `solana-test-validator`:
```
solana-test-validator --bpf-program SampGgdt3wioaoMZhC6LTSbg4pnuvQnSfJpDYeuXQBv ~/solana-cli-program-template/program/target/bpfel-unknown-unknown/release/solana_cli_template_program_bpf.so --ledger ~/solana-cli-program-template/.ledger --reset
```

## Invocations
You invoke all tests from the repo root directory:
* Full with output: `cargo test --test full -- --test-threads=1 --nocapture`
* Local with output: `cargo test --test thin -- --test-threads=1 --nocapture`
