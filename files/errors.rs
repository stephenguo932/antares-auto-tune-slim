//! Error types shared across Antares Auto Tune Slim modules.

use std::fmt;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    MissingSetting(&'static str),
    InvalidVersion(String),
    SignatureMismatch,
    RegistryWriteDenied,
    UpdateFailed { code: i32, stage: &'static str },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(e)                      => write!(f, "io: {}", e),
            Error::MissingSetting(k)          => write!(f, "missing setting: {}", k),
            Error::InvalidVersion(s)          => write!(f, "invalid version: {}", s),
            Error::SignatureMismatch          => write!(f, "signature mismatch"),
            Error::RegistryWriteDenied        => write!(f, "registry write denied"),
            Error::UpdateFailed { code, stage } => write!(f, "update failed at {} ({})", stage, code),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self { Error::Io(e) }
}

pub type Result<T> = std::result::Result<T, Error>;
