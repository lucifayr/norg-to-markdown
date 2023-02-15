use self::pre_process_lists::pre_process_lists;

pub mod pre_process_lists;

pub fn pre_process(text: &str) -> Result<String, regex::Error> {
    pre_process_lists(text)
}
