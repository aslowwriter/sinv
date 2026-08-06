pub(crate) mod sink;
pub(crate) mod source;
pub(crate) mod suggest;
pub(crate) mod write;

use clap::{Parser, Subcommand, ValueEnum};
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

    #[arg(long, short)]
    pub color: Option<ColorUse>,
}

#[derive(Debug, Subcommand, Clone, PartialEq)]
pub enum SubCommand {
    /// Write the data from the input file to the output file
    Write(WriteArgs),
    /// Search the inventory for close matches to `<search_term>`
    Suggest(SuggestArgs),
}

#[derive(Debug, Clone, Default, PartialEq, ValueEnum)]
pub(crate) enum ColorUse {
    Never,
    Always,
    #[default]
    Auto,
}

impl SubCommand {
    pub fn get_source(&self) -> DataSource {
        match self {
            SubCommand::Write(write_args) => write_args.source.clone().unwrap_or(DataSource::Stdin),
            SubCommand::Suggest(suggest_args) => suggest_args.source.clone(),
        }
    }
}
