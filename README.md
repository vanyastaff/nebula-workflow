## Code Style & Linting

This project uses the latest stable Rust toolchain. To ensure code quality and consistency, use the following commands:

```sh
# Format code
cargo fmt --all

# Run linter (clippy) with all warnings as errors
cargo clippy --all-targets --all-features -- -D warnings
```

To install required tools:

```sh
rustup update stable
rustup component add rustfmt
rustup component add clippy
``` 