use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO Error: {0}")]
    IO(#[from] Box<std::io::Error>),

    #[error("Internal error: {0}")]
    InternalError(String),
}

/// The result type used in this crate.
pub type Result<T> = std::result::Result<T, Error>;
