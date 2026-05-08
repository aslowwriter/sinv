use std::{
    fs::File,
    io::{Cursor, Read, Write},
    path::PathBuf,
};

use color_eyre::eyre::Result;
use reqwest::Url;
use sphinx_inv::{PlainTextSphinxInventoryWriter, SphinxInventoryReader, SphinxInventoryWriter};
use tracing::subscriber::set_global_default;

mod cli;
use crate::cli::{CliArgs, SubCommand};
use clap::Parser;

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = CliArgs::parse();
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(args.verbose.tracing_level_filter())
        .finish();

    set_global_default(subscriber)?;

    match args.cmd {
        SubCommand::Write(write_args) => {
            let input_reader: Box<dyn Read> = if let Ok(url) = Url::parse(&write_args.input) {
                tracing::info!("reading from url: {0}", write_args.input);
                let data = reqwest::blocking::get(url)?.bytes()?;
                let response_reader = Cursor::new(data);
                Box::new(response_reader)
            } else if &write_args.input == "-" {
                tracing::info!("reading from stdin");
                let stdin_reader = std::io::stdin();
                Box::new(stdin_reader)
            } else {
                tracing::info!("reading from file: {}", &write_args.input);
                let path_buf = PathBuf::from(&write_args.input);
                let file = File::create(path_buf)?;
                Box::new(file)
            };

            let mut output_writer: Box<dyn Write> = if &write_args.output == "-" {
                tracing::info!("writing to stdout");
                let stdout_writer = std::io::stdout();
                Box::new(stdout_writer)
            } else {
                tracing::info!("writing to file {}", &write_args.output);
                let path_buf = PathBuf::from(&write_args.output);
                let file = File::create(path_buf)?;
                Box::new(file)
            };

            let reader = SphinxInventoryReader::from_reader(input_reader)?;
            let header = reader.header().clone();

            match write_args.write_format {
                cli::Format::Plain => {
                    let mut writer = PlainTextSphinxInventoryWriter::from_header(header, 0, true);

                    for reference in reader {
                        match reference {
                            Ok(r) => writer.add_reference(r),
                            Err(e) => tracing::warn!("failed to parse line: {e}"),
                        }
                    }

                    writer.finalize(&mut output_writer)?;
                }
                cli::Format::Zlib => {
                    let mut writer = SphinxInventoryWriter::from_header(header, 0, true);
                    for reference in reader {
                        match reference {
                            Ok(r) => writer.add_reference(r),
                            Err(e) => tracing::warn!("failed to parse line: {e}"),
                        }
                    }

                    writer.finalize(&mut output_writer)?;
                }
            }
        }
        SubCommand::Suggest(_suggest_args) => todo!(),
    }

    // ...
    Ok(())
}
