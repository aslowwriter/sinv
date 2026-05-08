use clap::Args;

use clap::{Parser, Subcommand};
use clap_verbosity_flag::{LogLevel, Verbosity, VerbosityFilter};

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

#[derive(Debug, Clone, Args, PartialEq)]
pub struct WriteArgs {
    #[arg(short, long, action)]
    pub force: bool,
}

#[derive(Debug, Subcommand, Clone, PartialEq)]
pub enum SubCommand {
    /// Write the data from the input file to the output file
    Write(WriteArgs),
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::error::SinvError;
    use clap::Parser;

    #[test]
    fn test_args_write_file() -> Result<(), SinvError> {
        let args = CliArgs::parse_from(["sinv", "write"]);
        assert_eq!(args.cmd, SubCommand::Write(WriteArgs { force: false }));
        Ok(())
    }
}
