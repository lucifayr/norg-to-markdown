use regex::Regex;

use crate::regex::captures::lenght_of_nth_capture;
static TAB_WIDTH: usize = 4;

pub fn pre_process_lists(text: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(r"^(?P<spaces>\s*)([-~]{1,7})(?P<content>\s.*)$")?;
    let mut depth = 1;
    let mut spaces = 0;
    let mut above_is_list = false;

    let lines_with_depth: Vec<String> = text
        .lines()
        .map(|l| {
            if regex.captures(l).is_some() {
                let amount_of_spaces = lenght_of_nth_capture(&regex, l, 1) as i64;

                if amount_of_spaces - 2 >= spaces && above_is_list {
                    depth += 1;
                }

                spaces = amount_of_spaces;
                above_is_list = true
            } else {
                depth = 1;
                spaces = 0;
                above_is_list = false;
            }

            let depth = format!("[d-{depth}]");
            let list_suffix = match regex.captures(l) {
                Some(cap) => match cap.get(2) {
                    Some(mat) => mat.as_str().chars().next().unwrap_or('?'),
                    None => '?',
                },
                None => '?',
            };

            regex
                .replace(l, format!("{depth}$spaces{list_suffix}$content"))
                .to_string()
        })
        .collect();

    let regex = Regex::new(r"^\[d-(?P<depth>\d)\]\s*(?P<content>[-~]\s.*)$")?;
    let lines: Vec<String> = lines_with_depth
        .iter()
        .map(|l| {
            if let Some(cap) = regex.captures(l) {
                if let Some(mat) = cap.get(1) {
                    let depth = mat.as_str().parse::<usize>().unwrap_or(1);
                    let indent = vec![" "; (depth - 1) * TAB_WIDTH].join("");
                    return regex.replace(l, format!("{indent}$content")).to_string();
                }
            }

            l.to_owned()
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
        assert_eq!(pre_process_lists("      - item"), Ok("- item".to_owned()));
        assert_eq!(
            pre_process_lists("- item      - not an item"),
            Ok("- item      - not an item".to_owned())
        );
        assert_eq!(
            pre_process_lists("- item\n  - sub item"),
            Ok("- item\n    - sub item".to_owned())
        );
    }
}
