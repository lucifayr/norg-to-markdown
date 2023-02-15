use self::{
    parse_headings::parse_headings, parse_highlighting::parse_highlight, parse_links::parse_links,
    parse_ol::parse_ol, parse_todo::parse_todo,
};

pub mod parse_headings;
pub mod parse_highlighting;
pub mod parse_links;
pub mod parse_ol;
pub mod parse_todo;

pub fn parse(line: &str) -> Result<String, regex::Error> {
    let headings = parse_headings(line)?;
    let highlights = parse_highlight(&headings)?;
    let ol_list = parse_ol(&highlights)?;
    let links = parse_links(&ol_list)?;
    parse_todo(&links)
}
