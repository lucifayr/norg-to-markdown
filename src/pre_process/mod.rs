use self::{pre_process_indents::pre_process_indents, pre_process_lists::pre_process_lists};

pub mod pre_process_indents;
pub mod pre_process_lists;

pub fn pre_process(text: &str) -> Result<String, regex::Error> {
    let lists = pre_process_lists(text)?;
    pre_process_indents(&lists)
}
