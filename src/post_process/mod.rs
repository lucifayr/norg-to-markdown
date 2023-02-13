use regex::{Captures, Regex};

pub fn process(text: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(r"\[#{1, 6}\]")?;

    let mut prev = "".to_owned();
    let mut num = 1;

    let res = regex
        .replace_all(text, |caps: &Captures| {
            if let Some(cap) = caps.get(0) {
                let cap_str = cap.as_str().to_owned();
                if cap_str.eq(&prev) {
                    num += 1;
                } else {
                    num = 1;
                }

                prev = cap_str;
            }

            num.to_string()
        })
        .to_string();

    Ok(res)
}
