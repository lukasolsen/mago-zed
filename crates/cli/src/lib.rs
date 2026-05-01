pub mod diagnostics;
pub mod error;
pub mod executor;
pub mod parser;
pub mod runtime;

pub mod mago {
    pub mod command {
        use mago_core::terms::EDITOR_WORKSPACE_ROOT_ENV;
        use std::env;
        use std::path::{Path, PathBuf};
        use std::process::Command;

        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum MagoCommand {
            Lint,
            Analyze,
        }

        impl MagoCommand {
            #[must_use]
            pub const fn as_str(self) -> &'static str {
                match self {
                    Self::Lint => "lint",
                    Self::Analyze => "analyze",
                }
            }

            #[must_use]
            pub const fn diagnostics_pipeline() -> [Self; 1] {
                [Self::Lint]
            }
        }

        pub fn workspace_root_from_env() -> Option<PathBuf> {
            env::var_os(EDITOR_WORKSPACE_ROOT_ENV)
                .filter(|value| !value.is_empty())
                .map(PathBuf::from)
        }

        #[must_use]
        pub fn resolve_workspace_config(workspace_root: &Path) -> Option<PathBuf> {
            const FORMATS: [&str; 4] = ["toml", "yaml", "yml", "json"];

            for base_name in ["mago", "mago.dist"] {
                for format in FORMATS {
                    let candidate = workspace_root.join(format!("{base_name}.{format}"));
                    if candidate.is_file() {
                        return Some(candidate);
                    }
                }
            }

            None
        }

        #[must_use]
        pub fn build_mago_command(
            mago_bin: &str,
            file_path: &Path,
            command_type: MagoCommand,
            workspace_root: Option<&Path>,
        ) -> Command {
            let mut command = Command::new(mago_bin);

            if let Some(workspace_root) = workspace_root {
                command.arg("--workspace").arg(workspace_root);
                if let Some(config_path) = resolve_workspace_config(workspace_root) {
                    command.arg("--config").arg(config_path);
                }
                command.current_dir(workspace_root);
            }

            command.arg(command_type.as_str()).arg(file_path);
            command
        }
    }
}

pub use parser::parse_mago_output;
