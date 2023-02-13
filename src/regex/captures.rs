use regex::Regex;

pub fn lenght_of_nth_capture(regex: &Regex, line: &str, n: usize) -> usize {
    match regex.captures(line) {
        Some(cap) => match cap.get(n) {
            Some(mat) => mat.as_str().len(),
            None => 0,
        },
        None => 0,
    }
}
