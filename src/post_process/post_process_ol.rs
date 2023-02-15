use regex::{Captures, Regex};

pub fn post_process_ol(text: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(r"\[#{1, 6}\]")?;

    let mut prev = "".to_owned();
    let mut num = 1;
    let mut line_num = 1;
    let mut prev_line_num = 0;

    let lines: Vec<String> = text
        .lines()
        .map(|l| {
            line_num += 1;
            regex
                .replace_all(l, |caps: &Captures| {
                    if let Some(cap) = caps.get(0) {
                        let cap_str = cap.as_str().to_owned();
                        if cap_str.eq(&prev) && (line_num - prev_line_num) == 1 {
                            num += 1;
                        } else {
                            num = 1;
                        }

                        prev = cap_str;
                        prev_line_num = line_num;
                    }

                    num.to_string()
                })
                .to_string()
        })
        .collect();

    Ok(lines.join("\n"))
}
