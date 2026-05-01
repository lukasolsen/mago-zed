use crate::mago::command::workspace_root_from_env;
use mago_core::terms::{MAGO_BIN_DEFAULT, MAGO_BIN_ENV};
use std::env;
use std::io;
use std::path::PathBuf;
use tracing::{info, Level};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::util::SubscriberInitExt;

const LOG_FILE_NAME: &str = "mago-zed-wrapper.log";

/// Resolves the Mago executable for CLI command execution.
#[must_use]
pub fn resolve_mago_bin() -> String {
    env::var(MAGO_BIN_ENV)
        .ok()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| MAGO_BIN_DEFAULT.to_string())
}

/// Resolves the editor workspace root used for Mago process context.
#[must_use]
pub fn workspace_root_for_command() -> Option<PathBuf> {
    workspace_root_from_env().or_else(|| env::current_dir().ok())
}

/// # Errors
///
/// Returns an I/O error when current directory lookup or log file setup fails.
pub fn setup_logging() -> io::Result<WorkerGuard> {
    let log_dir = env::current_dir()?;
    let log_path = log_dir.join(LOG_FILE_NAME);
    let file_appender = tracing_appender::rolling::never(log_dir, LOG_FILE_NAME);
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .with_writer(non_blocking)
        .with_ansi(false)
        .with_target(true)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .finish()
        .init();

    info!(log_file = %log_path.display(), "==== file logging initialized");

    Ok(guard)
}
