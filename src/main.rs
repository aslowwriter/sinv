use tracing::subscriber::set_global_default;

mod cli;
mod error;
mod url;

use crate::{cli::CliArgs, error::SinvError};
use clap::Parser;

fn main() -> Result<(), SinvError> {
    let args = CliArgs::parse();

    let subscriber = tracing_subscriber::fmt()
        .with_max_level(args.verbose.tracing_level_filter())
        .finish();

    set_global_default(subscriber)?;

    Ok(())
}
