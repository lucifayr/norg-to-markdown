use std::{sync::mpsc::channel, thread};

use crate::{parse::parse, post_process::post_process, pre_process::pre_process};

pub fn convert(text: &str) -> Result<String, regex::Error> {
    let pre_processed = pre_process(text)?;

    let lines: Vec<String> = pre_processed.lines().map(|l| l.to_owned()).collect();

    let parsed = parse_lines_threaded(lines);
    post_process(&parsed)
}

fn parse_lines_threaded(lines: Vec<String>) -> String {
    let mut threads = vec![];
    let (sender, receiver) = channel();
    let chunks = lines.chunks(8);

    for (index, chunk) in chunks.enumerate() {
        let sender = sender.clone();
        let chunk = chunk.to_vec();
        threads.push(thread::spawn(move || {
            let parsed = chunk
                .into_iter()
                .map(|l| parse(&l).unwrap_or_else(|_| "???".to_owned()))
                .collect::<Vec<String>>()
                .join("\n")
                + "\n";

            sender
                .send((index, parsed))
                .expect("Failed to send value from thread");
        }));
    }

    let mut results = vec![];
    for _ in threads {
        results.push(
            receiver
                .recv()
                .expect("Failed to receive value from thread"),
        );
    }

    results.sort_by(|a, b| a.0.cmp(&b.0));
    results.iter().map(|(_, str)| str.to_owned()).collect()
}
