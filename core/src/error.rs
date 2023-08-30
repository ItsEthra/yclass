use serde::Serialize;
use std::{error, result};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Memflex: {0}")]
    Memflex(#[from] memflex::MfError),
    #[error("Failed to evaulate the expression")]
    AddrEval,
    #[error("Failed to find module {0}")]
    MissingModule(String),
    #[error("{0}")]
    Custom(Box<dyn error::Error>),
    #[error("Not attached to the process")]
    NotAttached,
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
