use clap::Args;

use crate::cli::source::DataSource;
#[derive(Debug, Clone, Args, PartialEq)]
pub struct SuggestArgs {
    /// The string to search for in all the provided inv files
    /// note that passing the search term from stdin is not supported
    /// supplying `-` here will cause sinv to search literally for the string '-'
    pub search_term: String,

    /// The sources to read the inventories from. These can be read from a file, a url, or from
    /// stdin. (note that stdin is not allowed to be specified multiple times)
    pub sources: Vec<DataSource>,

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
                sources: vec![DataSource::Path(PathBuf::from("bar"))],
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
                sources: vec![DataSource::Path(PathBuf::from("bar")),],
                search_term: String::from("foo"),
                threshold: None,
                max_items: None
            })
        );
    }
    #[test]
    fn test_args_suggest_file_url_and_stdin() {
        let args = CliArgs::parse_from(["sinv", "suggest", "foo", "https://example.org", "-"]);
        assert_eq!(
            args.cmd,
            SubCommand::Suggest(SuggestArgs {
                sources: vec![
                    DataSource::Url(Url::parse("https://example.org").unwrap()),
                    DataSource::Stdin,
                ],
                search_term: String::from("foo"),
                threshold: None,
                max_items: None
            })
        );
    }
    #[test]
    fn test_args_suggest_single_file_positional_search() {
        let args = CliArgs::parse_from(["sinv", "suggest", "foo", "bar"]);
        assert_eq!(
            args.cmd,
            SubCommand::Suggest(SuggestArgs {
                sources: vec![DataSource::Path(PathBuf::from("bar"))],
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
                sources: vec![DataSource::Stdin],
                search_term: String::from("foo"),
                threshold: None,
                max_items: None
            })
        );
    }
    #[test]
    fn test_args_multiple_stdin() {
        // This is just the parsing, we dont' want to allow this
        // but we'll catch it during input validation
        let args = CliArgs::parse_from(["sinv", "suggest", "-", "-", "bar", "baz"]);
        assert_eq!(
            args.cmd,
            SubCommand::Suggest(SuggestArgs {
                sources: vec![
                    DataSource::Stdin,
                    DataSource::Path(PathBuf::from("bar")),
                    DataSource::Path(PathBuf::from("baz")),
                ],
                search_term: String::from("-"),
                threshold: None,
                max_items: None
            })
        );
    }
    #[test]
    fn test_args_mixing_files_and_stdin_evil() {
        let args = CliArgs::parse_from(["sinv", "suggest", "arf", "foo", "-", "bar", "arf", "baz"]);
        assert_eq!(
            args.cmd,
            SubCommand::Suggest(SuggestArgs {
                sources: vec![
                    DataSource::Path(PathBuf::from("foo")),
                    DataSource::Stdin,
                    DataSource::Path(PathBuf::from("bar")),
                    DataSource::Path(PathBuf::from("arf")),
                    DataSource::Path(PathBuf::from("baz"))
                ],
                search_term: String::from("arf"),
                threshold: None,
                max_items: None
            })
        );
    }
    #[test]
    fn test_args_mixing_files_and_stdin() {
        let args = CliArgs::parse_from(["sinv", "suggest", "foo", "-", "bar", "baz"]);
        assert_eq!(
            args.cmd,
            SubCommand::Suggest(SuggestArgs {
                sources: vec![
                    DataSource::Stdin,
                    DataSource::Path(PathBuf::from("bar")),
                    DataSource::Path(PathBuf::from("baz"))
                ],
                search_term: String::from("foo"),
                threshold: None,
                max_items: None
            })
        );
    }
    #[test]
    fn test_args_suggest_multiple_files() {
        let args = CliArgs::parse_from(["sinv", "suggest", "foo", "bar", "baz"]);
        assert_eq!(
            args.cmd,
            SubCommand::Suggest(SuggestArgs {
                sources: vec![
                    DataSource::Path(PathBuf::from("bar")),
                    DataSource::Path(PathBuf::from("baz")),
                ],
                search_term: String::from("foo"),
                threshold: None,
                max_items: None
            })
        );
    }
}
