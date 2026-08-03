use clap::Args;

use crate::cli::source::DataSource;
#[derive(Debug, Clone, Args, PartialEq)]
pub struct CheckArgs {
    /// The sources to read the inventories from. These can be read from a file, a url, or from
    /// stdin. (note that stdin is not allowed to be specified multiple times)
    pub sources: Vec<DataSource>,
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
                sources: vec![DataSource::Path(PathBuf::from("foo"))]
            })
        );
    }
    #[test]
    fn test_args_lint_stdin() {
        let args = CliArgs::parse_from(["sinv", "check", "foo", "-"]);
        assert_eq!(
            args.cmd,
            SubCommand::Check(CheckArgs {
                sources: vec![DataSource::Path(PathBuf::from("foo")), DataSource::Stdin]
            })
        );
    }
    #[test]
    fn test_args_multiple_stdin() {
        // This is just the parsing, we dont' want to allow this
        // but we'll catch it during input validation
        let args = CliArgs::parse_from(["sinv", "check", "-", "-", "bar", "baz"]);
        assert_eq!(
            args.cmd,
            SubCommand::Check(CheckArgs {
                sources: vec![
                    DataSource::Stdin,
                    DataSource::Stdin,
                    DataSource::Path(PathBuf::from("bar")),
                    DataSource::Path(PathBuf::from("baz"))
                ]
            })
        );
    }
    #[test]
    fn test_args_mixing_files_and_stdin() {
        let args = CliArgs::parse_from(["sinv", "check", "foo", "-", "bar", "baz"]);
        assert_eq!(
            args.cmd,
            SubCommand::Check(CheckArgs {
                sources: vec![
                    DataSource::Path(PathBuf::from("foo")),
                    DataSource::Stdin,
                    DataSource::Path(PathBuf::from("bar")),
                    DataSource::Path(PathBuf::from("baz"))
                ]
            })
        );
    }
    #[test]
    fn test_args_lint_multiple_files() {
        let args = CliArgs::parse_from(["sinv", "check", "foo", "bar", "baz"]);
        assert_eq!(
            args.cmd,
            SubCommand::Check(CheckArgs {
                sources: vec![
                    DataSource::Path(PathBuf::from("foo")),
                    DataSource::Path(PathBuf::from("bar")),
                    DataSource::Path(PathBuf::from("baz"))
                ]
            })
        );
    }
}
