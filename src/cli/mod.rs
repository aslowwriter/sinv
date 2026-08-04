pub(crate) mod sink;
pub(crate) mod source;
pub(crate) mod suggest;
pub(crate) mod write;

use clap::{Parser, Subcommand};
use clap_verbosity_flag::{LogLevel, Verbosity, VerbosityFilter};

use crate::cli::source::DataSource;
use crate::cli::suggest::SuggestArgs;
use crate::cli::write::WriteArgs;

// don't have to construct it ourselves, tracing will just find it for us
#[allow(dead_code)]
pub struct CustomLogLevel {}

impl LogLevel for CustomLogLevel {
    fn default_filter() -> VerbosityFilter {
        VerbosityFilter::Error
    }
    fn quiet_help() -> Option<&'static str> {
        Some("suppress all logging output")
    }
    fn quiet_long_help() -> Option<&'static str> {
        Some("Suppress the logging output of the application, including errors.")
    }
    fn verbose_help() -> Option<&'static str> {
        Some("Increase verbosity of the logging (can be specified multiple times).")
    }
    fn verbose_long_help() -> Option<&'static str> {
        Some(
            "Increase the logging verbosity of the application by one level (ERROR, WARN, INFO, DEBUG, TRACE)",
        )
    }
}

#[derive(Parser)]
#[command(version, about, long_about= None)]
pub struct CliArgs {
    #[command(subcommand)]
    pub cmd: SubCommand,

    #[command(flatten)]
    pub verbose: Verbosity,
}

#[derive(Debug, Subcommand, Clone, PartialEq)]
pub enum SubCommand {
    /// Write the data from the input file to the output file
    Write(WriteArgs),
    Suggest(SuggestArgs),
}

impl SubCommand {
    pub fn get_source(&self) -> DataSource {
        match self {
            SubCommand::Write(write_args) => write_args.source.clone().unwrap_or(DataSource::Stdin),
            SubCommand::Suggest(suggest_args) => suggest_args.source.clone(),
        }
    }
}
