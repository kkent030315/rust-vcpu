use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader, Write},
    path::PathBuf,
};

use builder::{build_bytecode, Builder};
use clap::Parser;
use log::error;
use tracing_subscriber::EnvFilter;

#[cfg(test)]
mod test;

mod builder;
mod error;
mod lexer;
mod parser;

/// Command-line arguments for this compiler
#[derive(Parser, Debug)]
struct Args {
    /// The input source code
    ///
    /// The compiler can only proceed single source.
    #[arg(short, long)]
    source: PathBuf,
    /// Where to output the compiled VM bytecode
    #[arg(short, long)]
    out: PathBuf,
}

fn try_main() -> error::Result<()> {
    tracing_subscriber::fmt()
        .compact()
        .with_target(false)
        .with_file(false)
        .with_line_number(false)
        .without_time()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let args = Args::parse();

    let file = OpenOptions::new()
        .create(false)
        .write(false)
        .read(true)
        .open(args.source)?;
    let reader = BufReader::new(file);
    let mut builder = Builder::new();

    for line in reader.lines() {
        let line = format!("{}\n", line?);
        build_bytecode(line, &mut builder)?;
    }

    builder.finalize().map_err(|e| error::Error::Compile(e))?;

    let mut out_file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .read(false)
        .write(true)
        .open(args.out)?;
    let bytecode = builder.dump().map_err(|e| error::Error::Compile(e))?;
    out_file.write_all(&bytecode)?;

    Ok(())
}

fn main() {
    if let Err(err) = try_main() {
        error!("{err:?}")
    }
}
