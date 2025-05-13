use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO Error: {0}")]
    IO(#[from] Box<std::io::Error>),

    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("HTTP Error: {0}")]
    HTTPError(#[from] Box<reqwest::Error>),

    #[error("Invalid Status Code: {0}")]
    HTTPErrorWithStatusCode(reqwest::StatusCode),

    #[error("Deserialization Error: {0}")]
    Deserialization(String),
}

/// The result type used in this crate.
pub type Result<T> = std::result::Result<T, Error>;
