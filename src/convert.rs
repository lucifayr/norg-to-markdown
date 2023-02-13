use crate::{parse::parse, post_process::process};

pub fn convert(text: &str) -> Result<String, regex::Error> {
    let parsed: String = text
        .lines()
        .map(|l| parse(l).unwrap_or_else(|_| "???".to_owned()))
        .collect::<Vec<String>>()
        .join("\n");

    process(&parsed)
}
