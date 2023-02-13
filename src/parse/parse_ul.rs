use regex::Regex;

use crate::regex::captures::lenght_of_nth_capture;

pub fn parse_ul(line: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(r"^-{1,7}\s")?;
    let amount = lenght_of_nth_capture(&regex, line, 0);

    let arr = vec!["\t"; (amount as i64 - 1).clamp(0, i64::MAX) as usize];
    Ok(regex
        .replace(line, &format!("{}- ", arr.join("")))
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
        assert_eq!(parse_ul("-- item"), Ok("\t- item".to_owned()));
        assert_eq!(parse_ul("--- item"), Ok("\t\t- item".to_owned()));
        assert_eq!(parse_ul("--item"), Ok("--item".to_owned()));
        assert_eq!(parse_ul("---- item"), Ok("\t\t\t- item".to_owned()));
        assert_eq!(
            parse_ul("---------- item"),
            Ok("---------- item".to_owned())
        );
    }
}
