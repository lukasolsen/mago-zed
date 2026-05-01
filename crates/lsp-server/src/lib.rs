use mago_core::terms::{CLI_WRAPPER_BIN_DEFAULT, CLI_WRAPPER_BIN_ENV, EDITOR_WORKSPACE_ROOT_ENV};
use std::env;
use zed_extension_api::{self as zed, LanguageServerId, Result, Worktree};

/// Resolves the wrapper binary that hosts the tower-lsp server.
fn wrapper_binary() -> String {
    env::var(CLI_WRAPPER_BIN_ENV)
        .ok()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| CLI_WRAPPER_BIN_DEFAULT.to_string())
}

struct MagoZed {}

impl zed::Extension for MagoZed {
    fn new() -> Self {
        Self {}
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<zed::Command> {
        let wrapper_binary = wrapper_binary();

        Ok(zed::Command {
            command: wrapper_binary,
            args: vec![], // tower-lsp listens on stdio by default, no args needed
            env: vec![(EDITOR_WORKSPACE_ROOT_ENV.to_string(), worktree.root_path())],
        })
    }
}

zed::register_extension!(MagoZed);
