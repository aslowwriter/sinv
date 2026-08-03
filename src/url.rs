#![allow(clippy::unwrap_used, dead_code)]
use std::path::{Path, PathBuf};

use reqwest::Url;

#[derive(Debug)]
pub struct UrlPathIter {
    base_url: Url,
    remaining_path: Option<PathBuf>,
}

impl UrlPathIter {
    pub fn new(url: Url) -> UrlPathIter {
        let url_path = PathBuf::from(url.path());
        let remaining_path = if url_path.extension().is_some() {
            url_path.parent().map(Path::to_path_buf)
        } else {
            Some(url_path)
        };

        UrlPathIter {
            base_url: url,
            remaining_path,
        }
    }
}

impl Iterator for UrlPathIter {
    type Item = Url;

    fn next(&mut self) -> Option<Self::Item> {
        let p = self.remaining_path.take()?;

        let mut path = p.to_string_lossy().into_owned();
        if !path.ends_with('/') {
            path.push('/');
        }

        self.base_url.set_path(&path);
        self.remaining_path = p.parent().map(Path::to_path_buf);

        Some(self.base_url.join("objects.inv").unwrap())
    }
}

#[cfg(test)]
mod test {
    use reqwest::Url;

    use crate::url::UrlPathIter;

    #[test]
    pub fn url_discovery_with_ext() -> Result<(), url::ParseError> {
        let url = Url::parse(
            "https://docs.confluent.io/platform/current/streams/overview.html?session_ref=direct&url_ref=https%3A%2F%2Fdocs.confluent.io%2F",
        )?;

        let candidates: Vec<Url> = UrlPathIter::new(url).collect();

        assert_eq!(
            candidates,
            vec![
                Url::parse("https://docs.confluent.io/platform/current/streams/objects.inv")
                    .unwrap(),
                Url::parse("https://docs.confluent.io/platform/current/objects.inv").unwrap(),
                Url::parse("https://docs.confluent.io/platform/objects.inv").unwrap(),
                Url::parse("https://docs.confluent.io/objects.inv").unwrap(),
            ]
        );

        Ok(())
    }
    #[test]
    pub fn url_discovery_dir_trailing_slash() -> Result<(), url::ParseError> {
        let url = Url::parse("https://docs.aiohttp.org/en/stable/")?;
        let candidates: Vec<Url> = UrlPathIter::new(url).collect();

        assert_eq!(
            candidates,
            vec![
                Url::parse("https://docs.aiohttp.org/en/stable/objects.inv").unwrap(),
                Url::parse("https://docs.aiohttp.org/en/objects.inv").unwrap(),
                Url::parse("https://docs.aiohttp.org/objects.inv").unwrap(),
            ]
        );

        Ok(())
    }
    #[test]
    pub fn url_discovery_dir_no_trailing_slash() -> Result<(), url::ParseError> {
        let url = Url::parse("https://docs.aiohttp.org/en/stable")?;
        let candidates: Vec<Url> = UrlPathIter::new(url).collect();

        assert_eq!(
            candidates,
            vec![
                Url::parse("https://docs.aiohttp.org/en/stable/objects.inv").unwrap(),
                Url::parse("https://docs.aiohttp.org/en/objects.inv").unwrap(),
                Url::parse("https://docs.aiohttp.org/objects.inv").unwrap(),
            ]
        );

        Ok(())
    }
}
