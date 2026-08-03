use clap::Args;

use crate::cli::{sink::DataSink, source::DataSource};

#[derive(Debug, Clone, Args, PartialEq)]
pub struct WriteArgs {
    /// the source where the data will be read from, can be stdin, a file or a url.
    pub source: Option<DataSource>,
    /// the destination where the data will be written to, can be stdout or a file.
    pub sink: Option<DataSink>,

    /// overwrite any existing files at destination instead of erroring
    #[arg(short, long, action)]
    pub force: bool,
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use clap::Parser;
    use reqwest::Url;

    use crate::cli::{CliArgs, SubCommand, sink::DataSink, source::DataSource, write::WriteArgs};

    #[test]
    fn test_args_write_single_file() {
        let args = CliArgs::parse_from(["sinv", "write", "foo"]);
        assert_eq!(
            args.cmd,
            SubCommand::Write(WriteArgs {
                source: Some(DataSource::Path(PathBuf::from("foo"))),
                sink: None,
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
                force: false,
            })
        );
    }
    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        CliArgs::command().debug_assert();
    }

    #[test]
    fn test_args_write_url_as_sink() {
        let args = CliArgs::try_parse_from(["sinv", "write", "foo", "https://example.org"]);
        assert!(args.is_err());
    }
    #[test]
    fn test_args_write_json_mapping_incompatible_with_source() {
        let args = CliArgs::try_parse_from(["sinv", "write", "--json-mapping", "foo.json", "foo"]);
        assert!(args.is_err());
    }
}
