/// Canonical vocabulary shared across CLI and LSP-server crates.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorkspaceConcept {
    /// The workspace root path the editor opened for this project.
    EditorWorkspace,
}

impl WorkspaceConcept {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EditorWorkspace => "editor-workspace",
        }
    }
}

pub const EDITOR_WORKSPACE_ROOT_ENV: &str = "MAGO_WORKSPACE_ROOT";
pub const CLI_WRAPPER_BIN_ENV: &str = "MAGO_ZED_WRAPPER_BIN";
pub const CLI_WRAPPER_BIN_DEFAULT: &str = "cli";
pub const MAGO_BIN_ENV: &str = "MAGO_BIN";
pub const MAGO_BIN_DEFAULT: &str = "mago";
