use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Memflex: {0}")]
    MemflexError(#[from] memflex::MfError),
}

pub type Result<T> = std::result::Result<T, Error>;
