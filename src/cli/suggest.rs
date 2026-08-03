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
    pub threshold: Option<usize>,

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
                source: DataSource::Url(Url::parse("https://example.org").unwrap()),
                search_term: String::from("foo"),
                threshold: None,
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
                threshold: None,
                max_items: None
            })
        );
    }
}
