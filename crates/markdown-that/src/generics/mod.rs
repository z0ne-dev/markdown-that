//! Use these to build your own Markdown syntax.
//!
//! Some Markdown structures are very similar under the hood, for example
//!  - `*emphasis*`, `^supertext^` and `~~strikethrough~~`
//!  - `[link]()` and `![image]()`
//!
//! To reuse the code between all those, a notion of generic
//! Markdown structures was created. If you want to use syntax like
//! `=this=` or `++that++`, you only need to specify a character marker
//! and a renderer function; these rules will figure out the rest.
//!
pub mod inline;
