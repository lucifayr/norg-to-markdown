use regex::Regex;

pub fn post_process_code(text: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(r"(?ms:^@code(?P<lang> \w+)?.*?\n(?P<code>.*?)\n^@end)")?;
    Ok(regex.replace_all(text, "```$lang\n$code\n```").to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_post_process_code() {
        assert_eq!(post_process_code("text"), Ok("text".to_owned()));

        let eg = "
@code
const var = 12;
console.log(var);
@end
";
        let res = "
```
const var = 12;
console.log(var);
```
";

        assert_eq!(post_process_code(eg), Ok(res.to_owned()));

        let eg = "
@code rust
let var = 12;
println!(\"{var}\");
@end
";
        let res = "
``` rust
let var = 12;
println!(\"{var}\");
```
";

        assert_eq!(post_process_code(eg), Ok(res.to_owned()));

        let eg = "
@code rust
let var = 12;
println!(\"{var}\");
invalid@end
";
        let res = "
@code rust
let var = 12;
println!(\"{var}\");
invalid@end
";

        assert_eq!(post_process_code(eg), Ok(res.to_owned()));

        let eg = "
invalid@code rust
let var = 12;
println!(\"{var}\");
@end
";
        let res = "
invalid@code rust
let var = 12;
println!(\"{var}\");
@end
";

        assert_eq!(post_process_code(eg), Ok(res.to_owned()));
        let eg = "
@code rust valid
let var = 12;
println!(\"{var}\");
@end
";
        let res = "
``` rust
let var = 12;
println!(\"{var}\");
```
";

        assert_eq!(post_process_code(eg), Ok(res.to_owned()));
    }
}
