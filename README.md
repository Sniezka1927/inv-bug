# Prequesites

- cargo install contracts-node --git https://github.com/paritytech/substrate-contracts-node.git
- cargo +nightly contract build --release

# Run tests

```
cargo test --features e2e-tests -- --nocapture
```