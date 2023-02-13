use self::{
    parse_headings::parse_headings, parse_highlighting::parse_highlight, parse_ol::parse_ol,
    parse_ul::parse_ul,
};

pub mod parse_headings;
pub mod parse_highlighting;
pub mod parse_ol;
pub mod parse_ul;

pub fn parse(line: &str) -> Result<String, regex::Error> {
    let headings = parse_headings(line)?;
    let highlights = parse_highlight(&headings)?;
    let ul_list = parse_ul(&highlights)?;
    parse_ol(&ul_list)
}
