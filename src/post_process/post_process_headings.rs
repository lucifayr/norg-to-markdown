use regex::Regex;

pub fn post_process_headings(text: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(r"(?P<heading>#{1, 6}\s.+[\n\r])(?P<text>.+)")?;

    // This prevents sequential Headings from having inconsistent spacing
    // due to the 2. Heading being captured as text
    // e.g.:
    // # First Heading
    // ## Second Heading
    // ### First Heading
    // #### Second Heading
    // =====================>
    // - 1. Match
    // ------------------
    // # First Heading
    // ## Second Heading
    // - 2. Match
    // ------------------
    // ### First Heading
    // #### Second Heading
    let first_pass = regex.replace_all(text, "$heading\n$text").to_string();
    Ok(regex
        .replace_all(&first_pass, "$heading\n$text")
        .to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_post_process_headings() {
        let eg = "## Heading\nsome text without a newline above.";
        let res = "## Heading\n\nsome text without a newline above.";

        assert_eq!(post_process_headings(eg), Ok(res.to_owned()));

        let eg = "## Heading\n\nsome text with a newline above.";
        let res = "## Heading\n\nsome text with a newline above.";

        assert_eq!(post_process_headings(eg), Ok(res.to_owned()));

        let eg = "##Not Heading\nsome text that shouldn't get newline above.";
        let res = "##Not Heading\nsome text that shouldn't get newline above.";

        assert_eq!(post_process_headings(eg), Ok(res.to_owned()));
    }
}
