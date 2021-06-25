# Transaction testing

The programs transactions are tested using a lite-weight runtime framework `solana-program-test`.

You can run this from the command line or directly from the `lib.rs`

From the command line:
```
cargo test-bpf -- --test-threads=1 --nocapture
```
