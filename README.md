This is a fork of [markdown-it-rust](https://github.com/markdown-it-rust/markdown-it) with updated and maintained
crates.

Below is the original and unmodified README.

# markdown-it

[<img alt="web demo" src="https://img.shields.io/badge/demo-8da0cb?style=for-the-badge&labelColor=555555&logo=webpack&logoColor=white" height="20">](https://markdown-it-rust.github.io/markdown-it/)
[<img alt="github" src="https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/markdown-it-rust/markdown-it)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs-8da0cb?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/markdown-it)
[<img alt="crates.io" src="https://img.shields.io/crates/v/markdown-it.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/markdown-it)
[<img alt="coverage" src="https://img.shields.io/codecov/c/github/markdown-it-rust/markdown-it?style=for-the-badge" height="20">](https://app.codecov.io/gh/markdown-it-rust/markdown-it)

Rust port of popular [markdown-it.js](https://github.com/markdown-it/markdown-it) library.

TL;DR:

- if you want to get result *fast*, use [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark)
- if you want to render GFM exactly like github, use [comrak](https://github.com/kivikakk/comrak)
- if you want to define your own syntax (like `@mentions`, `:emoji:`, custom html classes), use this library

You can check a [demo](https://markdown-it-rust.github.io/markdown-it/) in your browser *(it's Rust compiled into
WASM)*.

### Features

- 100% CommonMark compatibility
- AST
- Source maps (full support, not just on block tags like cmark)
- Ability to write your own syntax of arbitrary complexity
    - to prove this point, CommonMark syntax itself is written as a plugin

### Usage

```rust
let parser = & mut markdown_that::MarkdownThat::new();
markdown_that::plugins::cmark::add(parser);
markdown_that::plugins::extra::add(parser);

let ast  = parser.parse("Hello **world**!");
let html = ast.render();

print!("{html}");
// prints "<p>Hello <strong>world</strong>!</p>"
```

For a guide on how to extend it, see `examples` folder.

### Notes

*This is an attempt at making a language-agnostic parser. You can probably parse AsciiDoc, reStructuredText
or [any other](https://github.com/mundimark/awesome-markdown-alternatives) plain text format with this without too much
effort. I&nbsp;might eventually write these as proof-of-concept.*
