use clap::Args;

use crate::cli::source::DataSource;
#[derive(Debug, Clone, Args, PartialEq)]
pub struct CheckArgs {
    /// The sources to read the inventories from. These can be read from a file, a url, or from
    /// stdin. (note that stdin is not allowed to be specified multiple times)
    pub source: DataSource,
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use clap::Parser;

    use crate::cli::{CliArgs, SubCommand, check::CheckArgs, source::DataSource};

    #[test]
    fn test_args_lint_single_file() {
        let args = CliArgs::parse_from(["sinv", "check", "foo"]);
        assert_eq!(
            args.cmd,
            SubCommand::Check(CheckArgs {
                source: DataSource::Path(PathBuf::from("foo"))
            })
        );
    }
    #[test]
    fn test_args_lint_url() {
        let args = CliArgs::parse_from(["sinv", "check", "https://example.org/foo/bar"]);
        assert_eq!(
            args.cmd,
            SubCommand::Check(CheckArgs {
                source: DataSource::Url(url::Url::parse("https://example.org/foo/bar").unwrap())
            })
        );
    }
    #[test]
    fn test_args_lint_stdin() {
        let args = CliArgs::parse_from(["sinv", "check", "-"]);
        assert_eq!(
            args.cmd,
            SubCommand::Check(CheckArgs {
                source: DataSource::Stdin
            })
        );
    }
}
