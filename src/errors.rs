use postgres::Error as PGError;
use snafu::Snafu;
use std::env::VarError;
use std::io::Error as IOError;
use std::path::PathBuf;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
    #[snafu(display("{}", message))]
    EnvErrorConfig { source: VarError, message: String },
    #[snafu(display("{}", message))]
    MigrationTypeConfig { message: String },
    #[snafu(display("{}", source))]
    DBErrorConfig { source: PGError },
    #[snafu(display("{} for given path {}", source, message))]
    DirError { source: IOError, message: String },
    #[snafu(display("Unable to read configuration from {}: {}", path.display(), source))]
    ReadConfiguration { source: IOError, path: PathBuf },
    #[snafu(display("{} {}", message, source))]
    MigrationFileConfig { source: PGError, message: String },
    #[snafu(display("{}", message))]
    CustomMessageError { message: String },
    NoneError

}

pub type Result<T, E = Error> = std::result::Result<T, E>;
