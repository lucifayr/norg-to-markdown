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
         -- sub sub sub bullet point 1
-fake bullet point

~ order list entry 1
~ order list entry 2

some text in between with bold italic */highlights/*.

~ level 1 order list entry 1
~ level 1 order list entry 2
   ~ level 2 order list entry 1

~ order list entry 1
- unorder list entry 1 with a {:link:}
~ order list entry 1

   ~~~ level 2 order list entry 1
   ~~~ level 2 order list entry 2
      ~~~~ level 3 order list entry 1

** To Do 
normal text.
{:a link to a file:}
/italic/ and *bold* text!";

        let res = convert(text).unwrap();
        insta::assert_snapshot!(res);
    }
}
