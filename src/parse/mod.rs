use self::{
    parse_headings::parse_headings,
    parse_highlighting::parse_highlight,
    parse_links::{parse_file_links, parse_web_links},
    parse_ol::parse_ol,
    parse_todo::parse_todo,
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
    let file_links = parse_file_links(&ol_list)?;
    let web_links = parse_web_links(&file_links)?;
    parse_todo(&web_links)
}
