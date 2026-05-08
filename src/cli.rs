use std::fmt::Display;

use clap::{Args, ValueEnum};

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

#[derive(Debug, Clone, ValueEnum)]
pub enum Format {
    Plain,
    Zlib,
}

impl Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Format::Plain => f.write_str("plain"),
            Format::Zlib => f.write_str("zlib"),
        }
    }
}

#[derive(Parser)]
#[command(version, about, long_about= None)]
pub struct CliArgs {
    #[command(subcommand)]
    pub cmd: SubCommand,

    #[arg(short, long)]
    pub overwrite: bool,

    #[command(flatten)]
    pub verbose: Verbosity,
}

#[derive(Debug, Clone, Args)]
pub struct WriteArgs {
    pub input: String,
    pub output: String,

    #[arg(short, long, action)]
    pub overwrite: bool,

    #[arg(default_value_t = Format::Zlib)]
    pub write_format: Format,
}

#[derive(Debug, Clone, Args)]
pub struct SuggestArgs {
    input: String,
    query: String,
}

#[derive(Debug, Subcommand, Clone)]
pub enum SubCommand {
    /// Write the data from the input file to the output file
    Write(WriteArgs),
    Suggest(SuggestArgs),
}
