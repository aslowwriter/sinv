use frizbee::{Config, Matcher};
use sphinx_inv::{
    InventoryHeader, SphinxInvError, SphinxInventoryReader, SphinxInventoryWriter, SphinxReference,
    WriteFormat,
};
use std::io::{ErrorKind, Write};
use std::{fs::File, io::stdout};
use tracing::subscriber::set_global_default;
use tracing::warn;

use owo_colors::{OwoColorize, Stream};

mod cli;
mod error;
mod url;

use crate::cli::suggest::SuggestArgs;
use crate::cli::write::WriteArgs;
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

fn read_refs_from_source(
    args: &CliArgs,
) -> Result<(InventoryHeader, Vec<SphinxReference>), SinvError> {
    let reader = args.cmd.get_source().into_reader()?;
    let inventory_reader = SphinxInventoryReader::from_reader(reader)?;
    let header = inventory_reader.header().clone();
    let mut references = Vec::with_capacity(80000);
    for reference in inventory_reader {
        match reference {
            Ok(r) => references.push(r),
            Err(SphinxInvError::ParseError(e)) => {
                warn!("{}", e);
            }
            Err(e) => Err(e)?,
        }
    }
    Ok((header, references))
}

fn handle_write(
    write_args: WriteArgs,
    header: InventoryHeader,
    references: Vec<SphinxReference>,
) -> Result<(), SinvError> {
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
            writer.finalize(
                &mut handler,
                &write_format,
                write_args.minified || write_format == WriteFormat::Zlib,
            )?;
        }
        DataSink::Path(path_buf) => {
            if path_buf.exists() && !write_args.force {
                return Err(SinvError::FileExists(path_buf));
            }
            let mut f = File::create(path_buf)?;
            writer.finalize(
                &mut f,
                &write_format,
                write_args.minified || write_format == WriteFormat::Zlib,
            )?;
        }
    }
    Ok(())
}

fn handle_suggest(
    suggest_args: SuggestArgs,
    references: &[SphinxReference],
) -> Result<(), SinvError> {
    let names: Vec<_> = references.iter().map(|r| &r.name).collect();
    let matcher_config = Config::default().sort(frizbee::SortStrategy::ScoreThenIndexDesc);

    let mut searcher = Matcher::new(suggest_args.search_term, &matcher_config);

    let mut matches = searcher.match_list_parallel(&names, 8);

    if let Some(thresh) = suggest_args.threshold {
        matches.retain(|m| m.score >= thresh);
    }

    if let Some(max_items) = suggest_args.max_items {
        matches.truncate(max_items);
    }
    let stdout = stdout();
    let mut stdout_handle = stdout.lock();

    match (suggest_args.sphinx_ref, suggest_args.only_matches) {
        (true, true) => {
            for m in matches {
                let reference = &references[m.index as usize];

                writeln!(
                    stdout_handle,
                    ":{}:`{}`",
                    reference
                        .sphinx_type
                        .if_supports_color(Stream::Stdout, |text| text.cyan()),
                    reference
                        .name
                        .if_supports_color(Stream::Stdout, |text| text.green()),
                )?;
            }
            Ok(())
        }

        (true, false) => {
            for m in matches {
                let reference = &references[m.index as usize];
                writeln!(
                    stdout_handle,
                    "{}|{}|:{}:`{}`",
                    m.score
                        .if_supports_color(Stream::Stdout, |text| text.dimmed()),
                    m.index
                        .if_supports_color(Stream::Stdout, |text| text.dimmed()),
                    reference
                        .sphinx_type
                        .if_supports_color(Stream::Stdout, |text| text.cyan()),
                    reference
                        .name
                        .if_supports_color(Stream::Stdout, |text| text.green()),
                )?;
            }
            Ok(())
        }
        (false, true) => {
            for m in matches {
                let reference = &references[m.index as usize];
                writeln!(
                    stdout_handle,
                    "{}|{}`",
                    reference
                        .sphinx_type
                        .if_supports_color(Stream::Stdout, |text| text.cyan()),
                    reference
                        .name
                        .if_supports_color(Stream::Stdout, |text| text.green()),
                )?;
            }
            Ok(())
        }
        (false, false) => {
            for m in matches {
                let reference = &references[m.index as usize];
                writeln!(
                    stdout_handle,
                    "{}|{}|{}|{}",
                    m.score
                        .if_supports_color(Stream::Stdout, |text| text.dimmed()),
                    m.index
                        .if_supports_color(Stream::Stdout, |text| text.dimmed()),
                    reference
                        .sphinx_type
                        .if_supports_color(Stream::Stdout, |text| text.cyan()),
                    reference
                        .name
                        .if_supports_color(Stream::Stdout, |text| text.green()),
                )?;
            }
            Ok(())
        }
    }
}

fn inner_main() -> Result<(), SinvError> {
    let args = CliArgs::parse();

    let subscriber = tracing_subscriber::fmt()
        .with_max_level(args.verbose.tracing_level_filter())
        .finish();

    set_global_default(subscriber)?;

    let (header, references) = read_refs_from_source(&args)?;

    match args.cmd {
        cli::SubCommand::Write(write_args) => handle_write(write_args, header, references)?,
        cli::SubCommand::Suggest(suggest_args) => handle_suggest(suggest_args, &references)?,
    }

    Ok(())
}
