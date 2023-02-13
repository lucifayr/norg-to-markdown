use std::{fs, io};

use clap::Parser;

use crate::{cli::args::Args, convert::convert};

mod cli;
mod convert;
mod parse;
mod post_process;
mod regex;

#[cfg(test)]
mod test;

fn main() -> Result<(), io::Error> {
    let args = Args::parse();

    let text = fs::read_to_string(&args.input)?;
    let res = match convert(&text) {
        Ok(res) => res,
        Err(err) => panic!("Failed to convert {} to markdown: {}", args.input, err),
    };

    let out_file = args.output.unwrap_or(format!("{}.md", args.input));
    fs::write(out_file, res)?;

    Ok(())
}
