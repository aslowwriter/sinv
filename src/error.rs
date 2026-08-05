use sphinx_inv::{SphinxInvError, SphinxParseError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SinvError {
    #[error("could not initialise logging")]
    Disconnect(#[from] tracing::dispatcher::SetGlobalDefaultError),

    #[error("Io error")]
    IoError(#[from] std::io::Error),

    #[error("HTTP request error")]
    HttpError(#[from] reqwest::Error),

    #[error("asldfkj;asdlfkjerror parsing line {} {} {}", .0.location, .0.message, .0.input)]
    ParseError(#[from] SphinxParseError),

    #[error("Header error: {0}")]
    InvalidHeader(SphinxInvError),

    #[error("Could not find inventory file anywhere in {0}")]
    NoInventoryFile(String),

    #[error("File already exists {0}")]
    FileExists(std::path::PathBuf),
}

impl From<SphinxInvError> for SinvError {
    fn from(value: SphinxInvError) -> Self {
        match value {
            SphinxInvError::IoError(error) => Self::IoError(error),
            SphinxInvError::ParseError(sphinx_parse_error) => Self::ParseError(sphinx_parse_error),
            e => Self::InvalidHeader(e),
        }
    }
}
