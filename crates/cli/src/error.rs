use crate::mago::command::MagoCommand;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::io;
use std::path::PathBuf;

#[derive(Debug)]
pub enum MagoError {
    ProcessSpawn {
        mago_bin: String,
        command_type: MagoCommand,
        file_path: PathBuf,
        source: io::Error,
    },
}

impl Display for MagoError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::ProcessSpawn { mago_bin, command_type, file_path, source } => write!(
                formatter,
                "failed to execute `{mago_bin} {command_type} {file_path}`: {source}",
                command_type = command_type.as_str(),
                file_path = file_path.display(),
            ),
        }
    }
}

impl Error for MagoError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::ProcessSpawn { source, .. } => Some(source),
        }
    }
}
