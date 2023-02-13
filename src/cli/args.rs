use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    // Input .norg file to convert
    #[arg(short, long)]
    pub input: String,

    // File to store the markdown output in
    #[arg(short, long)]
    pub output: Option<String>,
}
