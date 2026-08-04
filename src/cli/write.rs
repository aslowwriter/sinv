use clap::Args;
use sphinx_inv::WriteFormat;

use crate::cli::{sink::DataSink, source::DataSource};

#[derive(Debug, Clone, Args, PartialEq)]
pub struct WriteArgs {
    /// the source where the data will be read from, can be stdin, a file or a url.
    pub source: Option<DataSource>,
    /// the destination where the data will be written to, can be stdout or a file.
    pub sink: Option<DataSink>,

    #[arg(short, long, action)]
    pub minified: bool,

    /// what encoding should the output bit in?
    #[arg(short, long)]
    pub encoding: Option<OutputFormat>,

    /// overwrite any existing files at destination instead of erroring
    #[arg(short, long, action)]
    pub force: bool,
}

#[derive(Debug, Clone, PartialEq, clap::ValueEnum)]
pub enum OutputFormat {
    Plain,
    Zlib,
}

impl From<OutputFormat> for WriteFormat {
    fn from(value: OutputFormat) -> Self {
        match value {
            OutputFormat::Plain => WriteFormat::Plain,
            OutputFormat::Zlib => WriteFormat::Zlib,
        }
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use clap::Parser;

    use crate::cli::{CliArgs, SubCommand, sink::DataSink, source::DataSource, write::WriteArgs};

    #[test]
    fn test_args_write_single_file_plain_minified() {
        let args = CliArgs::parse_from(["sinv", "write", "foo", "-m", "-e", "plain"]);
        assert_eq!(
            args.cmd,
            SubCommand::Write(WriteArgs {
                source: Some(DataSource::Path(PathBuf::from("foo"))),
                encoding: Some(crate::cli::write::OutputFormat::Plain),
                minified: true,
                sink: None,
                force: false
            })
        );
    }
    #[test]
    fn test_args_write_single_file() {
        let args = CliArgs::parse_from(["sinv", "write", "foo"]);
        assert_eq!(
            args.cmd,
            SubCommand::Write(WriteArgs {
                source: Some(DataSource::Path(PathBuf::from("foo"))),
                encoding: None,
                minified: false,
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
                minified: false,
                encoding: None,
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
