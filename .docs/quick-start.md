# Quick Start

Build everything:

cargo build

Run tests:

cargo test --workspace

Run strict lint checks:

cargo clippy --workspace --all-targets --all-features -- -D warnings

Format code:

cargo fmt --all

Optional runtime variables:

- MAGO_ZED_WRAPPER_BIN
- MAGO_BIN
- MAGO_WORKSPACE_ROOT
