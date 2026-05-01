use crate::error::MagoError;
use crate::mago::command::{build_mago_command, resolve_workspace_config, MagoCommand};
use crate::parser::parse_mago_output;
use crate::runtime::workspace_root_for_command;
use mago_core::terms::EDITOR_WORKSPACE_ROOT_ENV;
use std::path::Path;
use tower_lsp::lsp_types::Diagnostic;
use tracing::{debug, info, warn};

/// Executes Mago commands for files and maps output into diagnostics.
pub struct MagoExecutor {
    mago_bin: String,
}

impl MagoExecutor {
    #[must_use]
    pub const fn new(mago_bin: String) -> Self {
        Self { mago_bin }
    }

    /// # Errors
    ///
    /// Returns a `MagoError` when the Mago process fails to spawn.
    pub fn run_check(
        &self,
        file_path: &Path,
        command_type: MagoCommand,
    ) -> Result<Vec<Diagnostic>, MagoError> {
        info!(command_type = command_type.as_str(), file_path = %file_path.display(), "==== running mago command");

        let workspace_root = workspace_root_for_command();
        let resolved_config = workspace_root.as_deref().and_then(resolve_workspace_config);

        debug!(
            command_type = command_type.as_str(),
            file_path = %file_path.display(),
            workspace_root_env = EDITOR_WORKSPACE_ROOT_ENV,
            workspace_root = ?workspace_root,
            resolved_config = ?resolved_config,
            "==== resolved workspace and workspace config"
        );

        let mut command =
            build_mago_command(&self.mago_bin, file_path, command_type, workspace_root.as_deref());
        let command_args: Vec<_> =
            command.get_args().map(|arg| arg.to_string_lossy().into_owned()).collect();
        let command_cwd = command
            .get_current_dir()
            .map_or_else(|| "<inherit>".to_string(), |path| path.display().to_string());

        info!(
            command_type = command_type.as_str(),
            file_path = %file_path.display(),
            mago_bin = %self.mago_bin,
            args = ?command_args,
            cwd = %command_cwd,
            "==== prepared mago process invocation"
        );

        let output = command.output().map_err(|source| MagoError::ProcessSpawn {
            mago_bin: self.mago_bin.clone(),
            command_type,
            file_path: file_path.to_path_buf(),
            source,
        })?;

        let status_code = output.status.code();
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        debug!(command_type = command_type.as_str(), file_path = %file_path.display(), ?status_code, success = output.status.success(), stdout_bytes = output.stdout.len(), stderr_bytes = output.stderr.len(), "==== mago process completed");

        if !stderr.is_empty() {
            warn!(
                command_type = command_type.as_str(),
                file_path = %file_path.display(),
                stderr = %stderr,
                "==== mago wrote stderr payload"
            );
        }

        if !output.status.success() {
            warn!(
                command_type = command_type.as_str(),
                file_path = %file_path.display(),
                ?status_code,
                "==== mago exited with non-zero status"
            );
        }

        let diagnostics = parse_mago_output(&stdout, command_type.as_str());

        debug!(command_type = command_type.as_str(), file_path = %file_path.display(), diagnostic_count = diagnostics.len(), "==== parsed diagnostics from mago output");

        Ok(diagnostics)
    }
}
