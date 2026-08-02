use std::io::{self};
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub enum DataSource {
    Stdin,
    Path(PathBuf),
}

#[derive(Debug, thiserror::Error)]
pub enum StdinError {
    // #[error("stdin read from more than once")]
    // StdInRepeatedUse,
    #[error(transparent)]
    StdIn(#[from] io::Error),
    // #[error("unable to parse from_str: {0}")]
    // FromStr(String),
}

impl FromStr for DataSource {
    type Err = StdinError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "-" => Ok(Self::Stdin),
            s => Ok(Self::Path(PathBuf::from(s))),
        }
    }
}
