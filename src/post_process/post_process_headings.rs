use regex::Regex;

pub fn post_process_headings(text: &str) -> Result<String, regex::Error> {
    let heading_regex = Regex::new(r"(?P<heading>#{1, 6}\s.+[\n\r])")?;
    let space_regex = Regex::new(r"([ \t]*[\n\r]){3,}")?;

    let spaced_text = heading_regex.replace_all(text, "\n$heading\n").to_string();
    Ok(space_regex.replace_all(&spaced_text, "\n\n").to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_post_process_headings() {
        let eg = "## Heading\nsome text without a newline above.";
        let res = "\n## Heading\n\nsome text without a newline above.";

        assert_eq!(post_process_headings(eg), Ok(res.to_owned()));

        let eg = "## Heading\n\nsome text with a newline above.";
        let res = "\n## Heading\n\nsome text with a newline above.";

        assert_eq!(post_process_headings(eg), Ok(res.to_owned()));

        let eg = "\n##Not Heading\nsome text that shouldn't get newline above.";
        let res = "\n##Not Heading\nsome text that shouldn't get newline above.";

        assert_eq!(post_process_headings(eg), Ok(res.to_owned()));
    }
}
