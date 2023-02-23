use std::{ffi::OsStr, fs, io, path::Path};

use clap::Parser;

use crate::{cli::args::Args, convert::convert};

mod cli;
mod convert;
mod parse;
mod post_process;
mod pre_process;
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

    let out_file = args.output.unwrap_or(format!(
        "{}.md",
        Path::new(args.input.split(".norg").next().unwrap_or("default.md"))
            .file_name()
            .unwrap_or(OsStr::new("default.md"))
            .to_str()
            .unwrap_or("default.md")
    ));
    fs::write(out_file, res)?;

    Ok(())
}
