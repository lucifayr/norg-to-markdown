use regex::Regex;

pub fn parse_web_links(line: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(r"\{(.+):\}")?;
    Ok(regex.replace_all(line, "[$1]($1)").to_string())
}
pub fn parse_file_links(line: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(r"\{:(.+):\}")?;
    Ok(regex.replace_all(line, "[$1]($1.md)").to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_web_links() {
        assert_eq!(parse_web_links("text"), Ok("text".to_owned()));
        assert_eq!(parse_web_links("- list item"), Ok("- list item".to_owned()));
        assert_eq!(parse_web_links("{link:}"), Ok("[link](link)".to_owned()));
        assert_eq!(parse_web_links(":{link}:"), Ok(":{link}:".to_owned()));
        assert_eq!(parse_web_links("{:link}:"), Ok("{:link}:".to_owned()));
    }

    #[test]
    fn test_parse_file_links() {
        assert_eq!(parse_file_links("text"), Ok("text".to_owned()));
        assert_eq!(
            parse_file_links("- list item"),
            Ok("- list item".to_owned())
        );
        assert_eq!(
            parse_file_links("{:link:}"),
            Ok("[link](link.md)".to_owned())
        );
        assert_eq!(parse_file_links(":{link}:"), Ok(":{link}:".to_owned()));
        assert_eq!(parse_file_links("{:link}:"), Ok("{:link}:".to_owned()));
    }
}
