use regex::Regex;

use crate::regex::captures::lenght_of_nth_capture;

pub fn parse_ol(line: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(r"^~{1,7}\s")?;
    let amount = lenght_of_nth_capture(&regex, line, 0);

    let tabs = vec!["\t"; (amount as i64 - 1).clamp(0, i64::MAX) as usize];
    let placeholder = vec!["#"; amount];
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
        assert_eq!(parse_ol("~~ item"), Ok(format!("\t[##]. item")));
        assert_eq!(parse_ol("~~~ item"), Ok(format!("\t\t[###]. item")));
        assert_eq!(parse_ol("~~item"), Ok("~~item".to_owned()));
        assert_eq!(parse_ol("~~~~ item"), Ok(format!("\t\t\t[####]. item")));
        assert_eq!(
            parse_ol("~~~~~~~~~~ item"),
            Ok("~~~~~~~~~~ item".to_owned())
        );
    }
}
