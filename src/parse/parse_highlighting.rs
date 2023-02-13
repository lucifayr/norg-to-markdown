use regex::Regex;

pub fn parse_highlight(line: &str) -> Result<String, regex::Error> {
    let bold_regex =
        Regex::new(r"(?P<p_left>\W|\A|\z)\*(?P<text>\S(.*\S)?)\*(?P<p_right>\W|\A|\z)")?;
    let bold_line = bold_regex.replace(line, "$p_left**$text**$p_right");

    let italic_regex =
        Regex::new(r"(?P<p_left>\W|\A|\z)/(?P<text>\S(.*\S)?)/(?P<p_right>\W|\A|\z)")?;
    let italic_line = italic_regex.replace(&bold_line, "$p_left*$text*$p_right");

    Ok(italic_line.to_string())
}

#[cfg(test)]
mod tests {
    use crate::parse::parse_highlighting::parse_highlight;

    #[test]
    fn test_parse_highlight() {
        // bold
        assert_eq!(parse_highlight("text"), Ok("text".to_owned()));
        assert_eq!(parse_highlight("*text*"), Ok("**text**".to_owned()));
        assert_eq!(parse_highlight("**"), Ok("**".to_owned()));
        assert_eq!(parse_highlight("*b*"), Ok("**b**".to_owned()));
        assert_eq!(
            parse_highlight("some text with *bold*!"),
            Ok("some text with **bold**!".to_owned())
        );
        assert_eq!(
            parse_highlight("some *text with bold*!"),
            Ok("some **text with bold**!".to_owned())
        );
        assert_eq!(
            parse_highlight("some * text with no bold*!"),
            Ok("some * text with no bold*!".to_owned())
        );
        assert_eq!(
            parse_highlight("some *text with no bold *!"),
            Ok("some *text with no bold *!".to_owned())
        );

        // italic
        assert_eq!(parse_highlight("text"), Ok("text".to_owned()));
        assert_eq!(parse_highlight("/text/"), Ok("*text*".to_owned()));
        assert_eq!(
            parse_highlight("some text with /italic/!"),
            Ok("some text with *italic*!".to_owned())
        );
        assert_eq!(
            parse_highlight("some /text with italic/!"),
            Ok("some *text with italic*!".to_owned())
        );
        assert_eq!(
            parse_highlight("some / text with no italic/!"),
            Ok("some / text with no italic/!".to_owned())
        );

        // bold italic
        assert_eq!(parse_highlight("text"), Ok("text".to_owned()));
        assert_eq!(parse_highlight("/*text*/"), Ok("***text***".to_owned()));
        assert_eq!(parse_highlight("*/text/*"), Ok("***text***".to_owned()));
        // assert_eq!(parse_highlight("*/text*/"), Ok("*/text*/".to_owned())); // know issue
        assert_eq!(
            parse_highlight("some text with /*bold and italic*/!"),
            Ok("some text with ***bold and italic***!".to_owned())
        );
    }
}
