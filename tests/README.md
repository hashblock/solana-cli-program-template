# solana-cli-template-program-bpf integration testing

This directory contains integration tests of the sample program (`solana-cli-program-template/program`).

The idea is to offer modularized test scanarios

## Scenarios
The following describe the individual modules and intent:
1. `solana-cli-program-template/tests/full.rs` (a.k.a. Full) - A "no-battery-required" integration tests that automatically loads the solana-test-validator, sample programs and any external keys. Each tests included in `full.rs`, in effect, run with a new clean *ledger* each time it is run.

## Invocations
* Full with output: `cargo test --test full -- --test-threads=1 --nocapture`
