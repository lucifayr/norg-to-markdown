use self::{post_process_code::post_process_code, post_process_ol::post_process_ol};

pub mod post_process_code;
pub mod post_process_ol;

pub fn post_process(text: &str) -> Result<String, regex::Error> {
    let ol = post_process_ol(text)?;
    post_process_code(&ol)
}
