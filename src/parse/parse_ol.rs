use regex::Regex;

use crate::regex::captures::lenght_of_nth_capture;

pub fn parse_ol(line: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(r"^(\s*)(~{1,7})\s")?;
    let amount = lenght_of_nth_capture(&regex, line, 1) / 3;

    let placeholder = vec!["#"; amount + 1];
    let tabs = vec!["\t"; amount];

    Ok(regex
        .replace(
            line,
            &format!("{}[{}]. ", tabs.join(""), placeholder.join("")),
        )
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
        assert_eq!(parse_ol("~~ item"), Ok(format!("[#]. item")));
        assert_eq!(parse_ol("   ~~ item"), Ok(format!("\t[##]. item")));
        assert_eq!(parse_ol("~~item"), Ok("~~item".to_owned()));
        assert_eq!(parse_ol("   ~~~~ item"), Ok(format!("\t[##]. item")));
        assert_eq!(
            parse_ol("~~~~~~~~~~ item"),
            Ok("~~~~~~~~~~ item".to_owned())
        );
    }
}
