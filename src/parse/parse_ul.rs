use regex::Regex;

use crate::regex::captures::lenght_of_nth_capture;

pub fn parse_ul(line: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(r"^(\s*)(-{1,7})\s")?;
    let amount = lenght_of_nth_capture(&regex, line, 1) / 3;

    let tabs = vec!["\t"; amount];

    Ok(regex
        .replace(line, &format!("{}- ", tabs.join("")))
        .to_string())
}

#[cfg(test)]
mod tests {
    use crate::parse::parse_ul::parse_ul;

    #[test]
    fn test_parse_ul() {
        assert_eq!(parse_ul("text"), Ok("text".to_owned()));
        assert_eq!(parse_ul("-item"), Ok("-item".to_owned()));
        assert_eq!(parse_ul("- item"), Ok("- item".to_owned()));
        assert_eq!(parse_ul("-- item"), Ok("- item".to_owned()));
        assert_eq!(parse_ul("   --- item"), Ok("\t- item".to_owned()));
        assert_eq!(parse_ul("      --- item"), Ok("\t\t- item".to_owned()));
        assert_eq!(parse_ul("      --item"), Ok("      --item".to_owned()));
        assert_eq!(parse_ul("---- item"), Ok("- item".to_owned()));
        assert_eq!(
            parse_ul("---------- item"),
            Ok("---------- item".to_owned())
        );
    }
}
