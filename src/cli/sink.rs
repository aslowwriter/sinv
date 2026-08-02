use std::io::{self};
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub enum DataSink {
    Stdout,
    Path(PathBuf),
}

#[derive(Debug, thiserror::Error)]
pub enum StdoutError {
    // #[error("stdin read from more than once")]
    // StdInRepeatedUse,
    #[error(transparent)]
    StdOut(#[from] io::Error),
    // #[error("unable to parse from_str: {0}")]
    // FromStr(String),
}

impl FromStr for DataSink {
    type Err = StdoutError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "-" => Ok(Self::Stdout),
            s => Ok(Self::Path(PathBuf::from(s))),
        }
    }
}
