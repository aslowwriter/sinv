use frizbee::{Config, Matcher};
use sphinx_inv::{
    SphinxInvError, SphinxInventoryReader, SphinxInventoryWriter, SphinxReference, WriteFormat,
};
use std::io::{ErrorKind, Write};
use std::{fs::File, io::stdout};
use tracing::subscriber::set_global_default;

mod cli;
mod error;
mod url;

use crate::{
    cli::{CliArgs, sink::DataSink},
    error::SinvError,
};
use clap::Parser;

fn main() -> Result<(), SinvError> {
    // not the most elegant of solutions, but we just want to exit
    // cleanly if the pipe is closed prematurely since that's perfectly acceptable behavior so
    // we have to do this little wrapper
    // if anything else goes wrong we'll just exit in whatever way
    // is appropriate

    let res = inner_main();
    if let Err(SinvError::IoError(ref e)) = res {
        if e.kind() == ErrorKind::BrokenPipe {
            Ok(())
        } else {
            res
        }
    } else {
        res
    }
}

fn inner_main() -> Result<(), SinvError> {
    let args = CliArgs::parse();

    let subscriber = tracing_subscriber::fmt()
        .with_max_level(args.verbose.tracing_level_filter())
        .finish();

    set_global_default(subscriber)?;

    let reader = args.cmd.get_source().into_reader()?;
    let inventory_reader = SphinxInventoryReader::from_reader(reader)?;
    let header = inventory_reader.header().clone();
    let references: Vec<_> = inventory_reader
        .collect::<Vec<Result<SphinxReference, SphinxInvError>>>()
        .into_iter()
        .filter_map(|r| match r {
            Ok(reference) => Some(reference),
            Err(e) => {
                eprintln!("failed to parse line: {e}");
                None
            }
        })
        .collect();

    match args.cmd {
        cli::SubCommand::Write(write_args) => {
            let mut writer = SphinxInventoryWriter::from_header(header, 0);
            for reference in references {
                writer.add_reference(reference);
            }

            let sink = write_args.sink.unwrap_or(DataSink::Stdout);
            let write_format: WriteFormat = write_args
                .encoding
                .unwrap_or(cli::write::OutputFormat::Zlib)
                .into();

            match sink {
                DataSink::Stdout => {
                    let stdout = stdout();
                    let mut handler = stdout.lock();
                    writer.finalize(&mut handler, &write_format, write_args.minified)?;
                }
                DataSink::Path(path_buf) => {
                    if path_buf.exists() && !write_args.force {
                        return Err(SinvError::FileExists(path_buf));
                    }
                    let mut f = File::create(path_buf)?;
                    writer.finalize(&mut f, &write_format, write_args.minified)?;
                }
            }
        }
        cli::SubCommand::Suggest(suggest_args) => {
            let names: Vec<_> = references.iter().map(|r| r.name.clone()).collect();
            let matcher_config = Config::default().sort(frizbee::SortStrategy::ScoreThenIndexDesc);

            let mut searcher = Matcher::new(suggest_args.search_term, &matcher_config);

            let mut matches = searcher.match_list_parallel(&names, 8);

            if let Some(thresh) = suggest_args.threshold {
                matches.retain(|m| m.score >= thresh);
            }

            if let Some(max_items) = suggest_args.max_items {
                matches = matches.into_iter().take(max_items).collect();
            }

            for m in matches {
                // unwrap is safe bc the searching should only index into the
                // refenrenes so should always be some
                #[allow(clippy::unwrap_used)]
                let reference = references.get(m.index as usize).unwrap();

                let stdout = stdout();
                let mut stdout_handle = stdout.lock();
                if suggest_args.sphinx_ref {
                    writeln!(
                        stdout_handle,
                        "{}|{}|:{}:`{}`",
                        m.score, m.index, reference.sphinx_type, reference.name,
                    )?;
                } else {
                    writeln!(
                        stdout_handle,
                        "{}|{}|{}|{}",
                        m.score, m.index, reference.sphinx_type, reference.name,
                    )?;
                }
            }
        }
    }

    Ok(())
}
