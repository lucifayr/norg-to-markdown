#[cfg(test)]
mod tests {
    use crate::convert::convert;

    #[test]
    fn test_convert() {
        let text = "
* heading
some text
   some text that shouldn't be indented

*fake heading
   - bullet point 1
   - bullet point 2
   - bullet point 3
   - *bold bullet point*
      - sub bullet point 1
         - sub sub bullet point 1
         - sub sub bullet point 2
            - sub sub sub bullet point 1
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

      ~ level 2 order list entry 1
      ~ level 2 order list entry 2
         ~ level 3 order list entry 1

** To Do 
normal text.
{:a link to a file:}
/italic/ and *bold* text!

** Checklist
   - () unchecked
   - (x) checked
   - (=) pending
   - (?) more info needed
   - (_) dropped
   - ( ) not done

@code
const var = 'this is some code';
console.log(var);
@end

@code rust
let var = \"this is some better code\";
println!(\"{var}\");
@end
";

        let res = convert(text).unwrap();
        insta::assert_snapshot!(res);
    }
}
