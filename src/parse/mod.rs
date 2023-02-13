use self::{parse_headings::parse_headings, parse_ul::parse_ul};

pub mod parse_headings;
pub mod parse_highlighting;
pub mod parse_ul;

pub fn parse(line: &str) -> Result<String, regex::Error> {
    let headings = parse_headings(line)?;
    parse_ul(&headings)
}

#[cfg(test)]
mod tests {
    use crate::parse::parse;

    #[test]
    fn test_parse() {
        let text = "
* heading
*fake heading
- bullet point 1
- bullet point 2
- bullet point 3
-- sub bullet point 1
--- sub sub bullet point 1
--- sub sub bullet point 2
---- sub sub bullet point 2
-fake bullet point

** to do 
nothing";

        let res: String = text
            .lines()
            .map(|l| parse(l).unwrap_or("???".to_owned()))
            .collect::<Vec<String>>()
            .join("\n");

        insta::assert_snapshot!(res);
    }
}
