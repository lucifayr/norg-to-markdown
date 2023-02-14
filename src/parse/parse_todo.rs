use regex::Regex;

pub fn parse_todo(line: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(r"^(\s*-{1,7}\s)\(([ x?!+=_-]?)\)(.*)")?;
    let status = match regex.captures(line) {
        Some(cap) => match cap.get(2) {
            Some(mat) => mat.as_str(),
            None => "",
        },
        None => "",
    };

    let done = if status == "x" { "x" } else { "" };
    let suffix = match status {
        " " => " (didn't do)",
        "?" => " (needs further input/clarification)",
        "!" => " (urgent)",
        "+" => " (recurring)",
        "-" => " (in-progress/pending)",
        "=" => " (on hold)",
        "_" => " (put down/cancelled)",
        _ => "",
    };

    Ok(regex
        .replace(line, format!("$1[{done}]$3{suffix}"))
        .to_string())
}

#[cfg(test)]
mod tests {
    use super::parse_todo;

    #[test]
    fn test_parse_todo() {
        assert_eq!(parse_todo("text"), Ok("text".to_owned()));
        assert_eq!(parse_todo("- item"), Ok("- item".to_owned()));
        assert_eq!(
            parse_todo("- () todo item"),
            Ok("- [] todo item".to_owned())
        );
        assert_eq!(
            parse_todo("- (x) todo item"),
            Ok("- [x] todo item".to_owned())
        );
        assert_eq!(
            parse_todo("- (_) todo item"),
            Ok("- [] todo item (put down/cancelled)".to_owned())
        );
        assert_eq!(
            parse_todo("- (=) todo item"),
            Ok("- [] todo item (on hold)".to_owned())
        );
        assert_eq!(
            parse_todo("- (r) todo item"),
            Ok("- (r) todo item".to_owned())
        );
    }
}
