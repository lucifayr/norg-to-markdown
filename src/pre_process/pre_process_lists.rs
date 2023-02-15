use regex::Regex;

use crate::regex::captures::lenght_of_nth_capture;

pub fn pre_process_lists(text: &str) -> Result<String, regex::Error> {
    let lines: Vec<String> = text
        .lines()
        .map(|l| {
            let regex = Regex::new(r"^(\s*)([-~]{1,7})\s").unwrap();
            let amount = (lenght_of_nth_capture(&regex, l, 1) / 3) as i64 - 1;
            let n = amount.clamp(0, i64::MAX) as usize;

            let tabs = vec!["    "; n].join("");
            let list_suffix = match regex.captures(l) {
                Some(cap) => match cap.get(2) {
                    Some(mat) => mat.as_str().chars().next().unwrap_or('?'),
                    None => '?',
                },
                None => '?',
            };

            regex
                .replace(l, format!("{tabs}{list_suffix} "))
                .to_string()
        })
        .collect();

    Ok(lines.join("\n"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pre_process_lists() {
        assert_eq!(pre_process_lists("text"), Ok("text".to_owned()));
        assert_eq!(pre_process_lists("- item"), Ok("- item".to_owned()));
        assert_eq!(pre_process_lists("   - item"), Ok("- item".to_owned()));
        assert_eq!(
            pre_process_lists("      - item"),
            Ok("    - item".to_owned())
        );
        assert_eq!(
            pre_process_lists("      - item      - not an item"),
            Ok("    - item      - not an item".to_owned())
        );
    }
}
