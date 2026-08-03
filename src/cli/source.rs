use std::path::PathBuf;
use std::str::FromStr;

use reqwest::Url;

use crate::error::SinvError;
use crate::url::UrlPathIter;

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
    StdIn(#[from] std::io::Error),
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

impl DataSource {
    pub fn into_reader(self) -> Result<Box<dyn std::io::Read>, SinvError> {
        match self {
            DataSource::Stdin => Ok(Box::new(std::io::stdin())),
            DataSource::Path(path_buf) => {
                let f = std::fs::File::open(path_buf)?;
                Ok(Box::new(f))
            }
            DataSource::Url(url) => {
                for candidate_url in UrlPathIter::new(url.clone()) {
                    let resp = reqwest::blocking::get(candidate_url)?;
                    if resp.status().is_success() {
                        return Ok(Box::new(resp));
                    }
                }

                Err(SinvError::NoInventoryFile(url.to_string()))
            }
        }
    }
}
