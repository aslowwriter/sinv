use std::io::{self};
use std::path::PathBuf;
use std::str::FromStr;

use reqwest::Url;

#[derive(Debug, Clone, PartialEq)]
pub enum DataSource {
    Stdin,
    Path(PathBuf),
    Url(Url),
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
        if s == "-" {
            Ok(Self::Stdin)
        } else if let Ok(url) = reqwest::Url::parse(s) {
            Ok(Self::Url(url))
        } else {
            Ok(Self::Path(PathBuf::from(s)))
        }
    }
}
