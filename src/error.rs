use thiserror::Error;

#[derive(Error, Debug)]
pub enum SinvError {
    #[error("could not initialise logging")]
    Disconnect(#[from] tracing::dispatcher::SetGlobalDefaultError),
}
