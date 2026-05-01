# Architecture

This workspace has three crates:

- crates/core: Shared terms and cross-crate constants.
- crates/cli: The tower-lsp server process and Mago command execution.
- crates/lsp-server: The Zed extension entrypoint that launches the CLI process.

Request flow:

1. Editor opens or saves a PHP file.
2. lsp-server starts CLI and passes editor workspace root via environment.
3. CLI runs Mago for the file.
4. CLI parses Mago output into diagnostics.
5. CLI publishes diagnostics back through LSP.

Core design rule:

All terminology shared by CLI and lsp-server is defined in crates/core.
