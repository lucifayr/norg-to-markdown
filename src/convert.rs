use crate::{parse::parse, post_process::post_process, pre_process::pre_process};

pub fn convert(text: &str) -> Result<String, regex::Error> {
    let pre_processed = pre_process(text)?;

    let parsed: String = pre_processed
        .lines()
        // I should fix this
        .map(|l| parse(l).unwrap_or_else(|_| "???".to_owned()))
        .collect::<Vec<String>>()
        .join("\n");

    post_process(&parsed)
}
