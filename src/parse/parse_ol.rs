use regex::Regex;

use crate::regex::captures::lenght_of_nth_capture;

pub fn parse_ol(line: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(r"^(\s*)~\s")?;
    let amount = lenght_of_nth_capture(&regex, line, 1) / 3;
    let placeholder = vec!["#"; amount + 1];

    Ok(regex
        .replace(line, &format!("$1[{}]. ", placeholder.join("")))
        .to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ul() {
        assert_eq!(parse_ol("text"), Ok("text".to_owned()));
        assert_eq!(parse_ol("~item"), Ok("~item".to_owned()));
        assert_eq!(parse_ol("~ item"), Ok(format!("[#]. item")));
        assert_eq!(parse_ol("~~ item"), Ok(format!("~~ item")));
        assert_eq!(parse_ol("   ~~ item"), Ok(format!("   ~~ item")));
        assert_eq!(parse_ol("~~item"), Ok("~~item".to_owned()));
        assert_eq!(parse_ol("   ~ item"), Ok(format!("   [##]. item")));
        assert_eq!(
            parse_ol("~~~~~~~~~~ item"),
            Ok("~~~~~~~~~~ item".to_owned())
        );
    }
}
