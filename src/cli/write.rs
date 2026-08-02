use std::path::PathBuf;

use clap::Args;

use crate::cli::{sink::DataSink, source::DataSource};

#[derive(Debug, Clone, Args, PartialEq)]
pub struct WriteArgs {
    pub source: Option<DataSource>,
    pub sink: Option<DataSink>,

    #[arg(short, long, action)]
    pub json_mapping: Option<PathBuf>,

    #[arg(short, long, action)]
    pub force: bool,
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use clap::Parser;

    use crate::cli::{CliArgs, SubCommand, sink::DataSink, source::DataSource, write::WriteArgs};

    #[test]
    fn test_args_write_single_file() {
        let args = CliArgs::parse_from(["sinv", "write", "foo"]);
        assert_eq!(
            args.cmd,
            SubCommand::Write(WriteArgs {
                source: Some(DataSource::Path(PathBuf::from("foo"))),
                sink: None,
                json_mapping: None,
                force: false
            })
        );
    }
    #[test]
    fn test_args_write_stdin() {
        let args = CliArgs::parse_from(["sinv", "write", "-", "-"]);
        assert_eq!(
            args.cmd,
            SubCommand::Write(WriteArgs {
                source: Some(DataSource::Stdin),
                sink: Some(DataSink::Stdout),
                json_mapping: None,
                force: false,
            })
        );
    }
    #[test]
    fn test_args_write_json_mapping() {
        let args = CliArgs::parse_from(["sinv", "write", "--json-mapping", "foo.json"]);
        assert_eq!(
            args.cmd,
            SubCommand::Write(WriteArgs {
                source: None,
                sink: None,
                json_mapping: Some(PathBuf::from("foo.json")),
                force: false,
            })
        );
    }
}
