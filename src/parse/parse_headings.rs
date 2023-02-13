use regex::Regex;

pub fn replace_headings(line: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(r"(?m:^\*{1,5}\s)")?;
    let amount = match regex.captures(line) {
        Some(cap) => match cap.get(0) {
            Some(mat) => mat.as_str().trim_end().len(),
            None => 0,
        },
        None => 0,
    };

    let arr = vec!["#"; amount];
    Ok(regex.replace(line, arr.join("") + " ").to_string())
}

#[cfg(test)]
mod tests {
    use super::replace_headings;

    #[test]
    fn test_replace_headings() {
        assert_eq!(replace_headings("text"), Ok("text".to_owned()));
        assert_eq!(replace_headings("* heading1"), Ok("# heading1".to_owned()));
        assert_eq!(replace_headings("*heading2"), Ok("*heading2".to_owned()));
        assert_eq!(
            replace_headings("** heading large"),
            Ok("## heading large".to_owned())
        );
        assert_eq!(
            replace_headings("not a * heading"),
            Ok("not a * heading".to_owned())
        );

        let text = "
* heading
    - bullet point 1
    - bullet point 2
    - bullet point 3
** to do 
nothing";

        let res: String = text
            .lines()
            .map(|l| replace_headings(l).unwrap_or("???".to_owned()))
            .collect::<Vec<String>>()
            .join("\n");

        insta::assert_snapshot!(res);
    }
}
