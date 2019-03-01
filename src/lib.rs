extern crate pulldown_cmark;

use pulldown_cmark::Event::{Text, SoftBreak, HardBreak};
use pulldown_cmark::Parser;

pub fn strip_markdown(markdown: &str) -> String {
    let mut parser = Parser::new(&markdown);
    let mut buffer = String::new();
    while let Some(event) = parser.next() {
        match event {
            Text(text) => buffer.push_str(&text),
            SoftBreak => buffer.push('\n'),
            HardBreak => buffer.push_str("\n \n"),
            _ => ()
        }
    }
    buffer
}



#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn basic_inline_matches() {
    let markdown_str = r#"
hello
=====
* alpha
* beta
"#;
    let expected = r#"
hello

alpha
beta
"#;
        assert_eq!(strip_markdown(markdown_str), expected);
    }
}
