use regex::Regex;

pub fn parse_highlight(line: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(r"")?;

    todo!()
}

#[cfg(test)]
mod tests {
    use crate::parse::parse_highlighting::parse_highlight;

    #[test]
    fn test_parse_highlight() {
        // bold
        assert_eq!(parse_highlight("text"), Ok("text".to_owned()));
        assert_eq!(parse_highlight("*text*"), Ok("**text**".to_owned()));
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
            Ok("some * text with bold*!".to_owned())
        );

        // italic
        assert_eq!(parse_highlight("text"), Ok("text".to_owned()));
        assert_eq!(parse_highlight("/text/"), Ok("*text*".to_owned()));
        assert_eq!(
            parse_highlight("some text with /bold/!"),
            Ok("some text with *bold*!".to_owned())
        );
        assert_eq!(
            parse_highlight("some /text with bold/!"),
            Ok("some *text with bold*!".to_owned())
        );
        assert_eq!(
            parse_highlight("some / text with no bold/!"),
            Ok("some / text with bold/!".to_owned())
        );

        // bold italic
        assert_eq!(parse_highlight("text"), Ok("text".to_owned()));
        assert_eq!(parse_highlight("/*text*/"), Ok("***text***".to_owned()));
        assert_eq!(parse_highlight("*/text/*"), Ok("***text***".to_owned()));
        assert_eq!(parse_highlight("*/text*/"), Ok("*/text*/".to_owned()));
        assert_eq!(
            parse_highlight("some text with /bold/!"),
            Ok("some text with *bold*!".to_owned())
        );
    }
}
