use sphinx_inv::SphinxInvError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SinvError {
    #[error("could not initialise logging")]
    Disconnect(#[from] tracing::dispatcher::SetGlobalDefaultError),

    #[error("Io error")]
    IoError(#[from] std::io::Error),

    #[error("HTTP request error")]
    HttpError(#[from] reqwest::Error),
    #[error("Parse error request error")]
    InvError(#[from] SphinxInvError),

    #[error("Could not find inventory file anywhere in {0}")]
    NoInventoryFile(String),

    #[error("File already exists {0}")]
    FileExists(std::path::PathBuf),
}
