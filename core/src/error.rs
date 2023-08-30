use serde::Serialize;
use std::result;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Memflex: {0}")]
    MemflexError(#[from] memflex::MfError),
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        format!("{self}").serialize(serializer)
    }
}

pub type Result<T> = result::Result<T, Error>;
