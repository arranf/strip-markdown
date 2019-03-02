# Strip Markdown

This is a Rust library to remove markdown.
Behind the scenes it parses markdown using the [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark) [crate](https://crates.io/crates/pulldown-cmark).

## Usage
```rust
extern crate strip_markdown;
use strip_markdown::*;

let stripped = strip_markdown(&my_markdown);
```