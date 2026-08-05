pub(crate) mod sink;
pub(crate) mod source;
pub(crate) mod suggest;
pub(crate) mod write;

use clap::{Parser, Subcommand};
use clap_verbosity_flag::Verbosity;

use crate::cli::source::DataSource;
use crate::cli::suggest::SuggestArgs;
use crate::cli::write::WriteArgs;

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
