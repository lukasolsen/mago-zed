# Encyclopedia

## Terms

Editor Workspace

- Meaning: The workspace root opened in the editor.
- Constant: EDITOR_WORKSPACE_ROOT_ENV
- Value: MAGO_WORKSPACE_ROOT

CLI Wrapper Binary

- Meaning: Executable launched by the Zed extension.
- Constants: CLI_WRAPPER_BIN_ENV, CLI_WRAPPER_BIN_DEFAULT

Mago Binary

- Meaning: Executable used by the CLI executor.
- Constants: MAGO_BIN_ENV, MAGO_BIN_DEFAULT

## Principles

- Shared terms live in crates/core.
- CLI and lsp-server must not duplicate shared constants.
- Error messages remain stable unless behavior intentionally changes.
