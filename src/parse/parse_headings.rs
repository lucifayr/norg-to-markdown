use regex::Regex;

use crate::regex::captures::lenght_of_nth_capture;

pub fn parse_headings(line: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(r"(?m:^\*{1,5}\s)")?;
    let amount = lenght_of_nth_capture(&regex, line, 0);

    let arr = vec!["#"; amount];
    Ok(regex.replace(line, arr.join("") + " ").to_string())
}

#[cfg(test)]
mod tests {
    use super::parse_headings;

    #[test]
    fn test_parse_headings() {
        assert_eq!(parse_headings("text"), Ok("text".to_owned()));
        assert_eq!(parse_headings("* heading1"), Ok("# heading1".to_owned()));
        assert_eq!(parse_headings("*heading2"), Ok("*heading2".to_owned()));
        assert_eq!(
            parse_headings("** heading large"),
            Ok("## heading large".to_owned())
        );
        assert_eq!(
            parse_headings("not a * heading"),
            Ok("not a * heading".to_owned())
        );
    }
}
