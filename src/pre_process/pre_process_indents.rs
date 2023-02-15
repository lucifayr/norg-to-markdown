use regex::Regex;

pub fn pre_process_indents(text: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(r"(?m:^\s*(?P<content>[^-~ ].*$))")?;
    Ok(regex.replace_all(text, "$content").to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pre_process_indents() {
        assert_eq!(pre_process_indents("text"), Ok("text".to_owned()));
        assert_eq!(
            pre_process_indents("    indented text"),
            Ok("indented text".to_owned())
        );
        assert_eq!(
            pre_process_indents("    {:indented link:}"),
            Ok("{:indented link:}".to_owned())
        );
        assert_eq!(
            pre_process_indents("    - ul item"),
            Ok("    - ul item".to_owned())
        );
        // know issue
        // assert_eq!(
        //     pre_process_indents("    -not ul item"),
        //     Ok("-not ul item".to_owned())
        // );
        assert_eq!(
            pre_process_indents("    ~ ol item"),
            Ok("    ~ ol item".to_owned())
        );
    }
}
