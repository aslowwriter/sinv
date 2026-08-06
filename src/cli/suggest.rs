use clap::Args;

use crate::cli::source::DataSource;
#[derive(Debug, Clone, Args, PartialEq)]
pub struct SuggestArgs {
    /// The string to search for in all the provided inv files
    /// note that passing the search term from stdin is not supported
    /// supplying `-` here will cause sinv to search literally for the string '-'
    pub search_term: String,

    /// The source to read the inventory from. These can be read from a file, a url, or from
    /// stdin.
    pub source: DataSource,

    /// The minimum score needed to return a suggestion
    #[arg(short, long)]
    pub threshold: Option<u16>,

    #[arg(short, long)]
    pub sphinx_ref: bool,

    /// show only the matches and not the scoring or index
    #[arg(short, long)]
    pub only_matches: bool,

    /// The maximum number of items to return, if unset, all will be returned.
    #[arg(short, long)]
    pub max_items: Option<usize>,
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use clap::Parser;
    use reqwest::Url;

    use crate::cli::{CliArgs, SubCommand, source::DataSource, suggest::SuggestArgs};

    #[test]
    fn test_args_suggest_single_file_with_thresh_and_max() {
        let args = CliArgs::parse_from(["sinv", "suggest", "foo", "bar", "-t", "50", "-m", "3"]);
        assert_eq!(
            args.cmd,
            SubCommand::Suggest(SuggestArgs {
                source: DataSource::Path(PathBuf::from("bar")),
                search_term: String::from("foo"),
                threshold: Some(50),
                only_matches: false,
                sphinx_ref: false,
                max_items: Some(3)
            })
        );
    }
    #[test]
    fn test_args_suggest_single_file_named_search() {
        let args = CliArgs::parse_from(["sinv", "suggest", "foo", "bar"]);
        assert_eq!(
            args.cmd,
            SubCommand::Suggest(SuggestArgs {
                source: DataSource::Path(PathBuf::from("bar")),
                search_term: String::from("foo"),
                sphinx_ref: false,
                only_matches: false,
                threshold: None,
                max_items: None
            })
        );
    }
    #[test]
    fn test_args_suggest_url() {
        let args = CliArgs::parse_from(["sinv", "suggest", "foo", "https://example.org"]);
        assert_eq!(
            args.cmd,
            SubCommand::Suggest(SuggestArgs {
                // known good url so unwrap is safe
                #[allow(clippy::unwrap_used)]
                source: DataSource::Url(Url::parse("https://example.org").unwrap()),
                search_term: String::from("foo"),
                sphinx_ref: false,
                threshold: None,
                only_matches: false,
                max_items: None
            })
        );
    }
    #[test]
    fn test_args_suggest_stdin() {
        let args = CliArgs::parse_from(["sinv", "suggest", "foo", "-"]);
        assert_eq!(
            args.cmd,
            SubCommand::Suggest(SuggestArgs {
                source: DataSource::Stdin,
                search_term: String::from("foo"),
                sphinx_ref: false,
                only_matches: false,
                threshold: None,
                max_items: None
            })
        );
    }
}
