//! Ready-to-use plugins. Everything, including basic markdown syntax, is a plugin.
//!
//! This library is made to be as extensible as possible. In order to ensure that
//! you can write your own markdown syntax of any arbitrary complexity,
//! CommonMark syntax itself is made into a plugin (`cmark`), which you can use
//! as an example of how to write your own.
//!
//! Add each plugin you need by invoking `add` function like this:
//! ```rust
//! let md = &mut markdown_that::MarkdownThat::new();
//! markdown_that::plugins::cmark::add(md);
//! markdown_that::plugins::extra::add(md);
//! markdown_that::plugins::html::add(md);
//! markdown_that::plugins::sourcepos::add(md);
//! // ...
//! ```
pub mod cmark;
pub mod extra;
pub mod html;
pub mod sourcepos;
