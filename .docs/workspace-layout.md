# Workspace Layout

Top-level:

- crates/core: Shared vocabulary and constants.
- crates/cli: LSP runtime, executor, parser, diagnostics mapping.
- crates/lsp-server: Zed extension glue.

CLI internal modules:

- runtime: Process runtime setup and environment resolution.
- executor: Mago command process execution.
- parser: Mago text output parsing.
- diagnostics: LSP Diagnostic construction.
- mago/command: Command building and config discovery.

Shared term source:

- crates/core/src/terms.rs
