# Contributing

The following commands will ensure the unit test cases are executed, and any issues with code
formatting or style are highlighted before pushing commits.

```sh
cargo test
cargo fmt --all -- --check
cargo clippy --workspace --tests -- --deny warnings
```

Running `cargo fmt --all` will fix code formatting.
