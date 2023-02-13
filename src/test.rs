#[cfg(test)]
mod tests {
    use crate::convert::convert;

    #[test]
    fn test_convert() {
        let text = "
* heading
*fake heading
- bullet point 1
- bullet point 2
- bullet point 3
- *bold bullet point*
-- sub bullet point 1
--- sub sub bullet point 1
--- sub sub bullet point 2
---- sub sub bullet point 2
-fake bullet point

~ order list entry 1
~ order list entry 2

some text in between with bold italic */highlights/*.

~~~ level 3 order list entry 1
~~~ level 3 order list entry 2
~~~~ level 4 order list entry 1

** to do 
nothing
/italic/ and *bold* text!";

        // let res: String = text
        //     .lines()
        //     .map(|l| parse(l).unwrap_or("???".to_owned()))
        //     .collect::<Vec<String>>()
        //     .join("\n");

        let res = convert(text).unwrap();
        insta::assert_snapshot!(res);
    }
}
