use clap::Args;

use crate::cli::source::DataSource;

#[derive(Debug, Clone, Args, PartialEq)]
pub struct WriteArgs {
    pub source: DataSource,

    #[arg(short, long, action)]
    pub force: bool,
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use clap::Parser;

    use crate::cli::{CliArgs, SubCommand, source::DataSource, write::WriteArgs};

    #[test]
    fn test_args_write_single_file() {
        let args = CliArgs::parse_from(["sinv", "write", "foo"]);
        assert_eq!(
            args.cmd,
            SubCommand::Write(WriteArgs {
                source: DataSource::Path(PathBuf::from("foo")),
                force: false
            })
        );
    }
    #[test]
    fn test_args_write_stdin() {
        let args = CliArgs::parse_from(["sinv", "write", "-"]);
        assert_eq!(
            args.cmd,
            SubCommand::Write(WriteArgs {
                source: DataSource::Stdin,
                force: false,
            })
        );
    }
}
