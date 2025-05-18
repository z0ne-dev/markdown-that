//! Regexps to match html elements
//!
#![allow(non_upper_case_globals)]

use const_format::formatcp;
use regex::Regex;
use std::sync::LazyLock;

const attr_name: &str = r#"[a-zA-Z_:][a-zA-Z0-9:._-]*"#;

const unquoted: &str = r#"[^"'=<>`\x00-\x20]+"#;
const single_quoted: &str = r#"'[^']*'"#;
const double_quoted: &str = r#""[^"]*""#;

const attr_value: &str = formatcp!("(?:{unquoted}|{single_quoted}|{double_quoted})");

const attribute: &str = formatcp!("(?:\\s+{attr_name}(?:\\s*=\\s*{attr_value})?)");

const open_tag: &str = formatcp!("<[A-Za-z][A-Za-z0-9\\-]*{attribute}*\\s*/?>");

const close_tag: &str = r#"</[A-Za-z][A-Za-z0-9\-]*\s*>"#;
const comment: &str = r#"<!---->|<!--(?:-?[^>-])(?:-?[^-])*-->"#;
const processing: &str = r#"<[?][\s\S]*?[?]>"#;
const declaration: &str = r#"<![A-Z]+\s+[^>]*>"#;
const cdata: &str = r#"<!\[CDATA\[[\s\S]*?\]\]>"#;

#[allow(clippy::double_parens)]
pub static HTML_TAG_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(formatcp!(
        "^(?:{open_tag}|{close_tag}|{comment}|{processing}|{declaration}|{cdata})"
    ))
    .unwrap()
});

#[allow(clippy::double_parens)]
pub static HTML_OPEN_CLOSE_TAG_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(formatcp!("^(?:{open_tag}|{close_tag})")).unwrap());

pub static HTML_LINK_OPEN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"^<a[>\s]"#).unwrap());

pub static HTML_LINK_CLOSE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"^</a\s*>"#).unwrap());
