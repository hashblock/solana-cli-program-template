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
1. `tests/full.rs` (a.k.a. Full) - A "no-battery-required" integration tests that automatically loads `solana-test-validator`, the sample programs and any external keys. Each tests included in `full.rs`, in effect, run with a new clean *ledger* each time it is run.

## Invocations
You invoke all tests from the repo root directory:
* Full with output: `cargo test --test full -- --test-threads=1 --nocapture`
