//! Link reference definition
//!
//! `[label]: /url "title"`
//!
//! <https://spec.commonmark.org/0.30/#link-reference-definition>
//!
//! This plugin parses Markdown link references. Check the documentation on [ReferenceMap]
//! to see how you can use and/or extend it if you have an external source for references.
//!
use crate::common::utils::normalize_reference;
use crate::generics::inline::full_link;
use crate::parser::block::{BlockRule, BlockState};
use crate::parser::extset::RootExt;
use crate::{MarkdownThat, Node, NodeValue};
use downcast_rs::{Downcast, impl_downcast};
use educe::Educe;
use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

/// Storage for parsed references
///
/// If you have some external source for your link references, you can add them like this:
///
/// ```rust
/// use markdown_that::parser::block::builtin::BlockParserRule;
/// use markdown_that::parser::core::{CoreRule, Root};
/// use markdown_that::plugins::cmark::block::reference::{ReferenceMap, DefaultReferenceMap, CustomReferenceMap};
/// use markdown_that::{MarkdownThat, Node};
///
/// let md = &mut MarkdownThat::new();
/// markdown_that::plugins::cmark::add(md);
///
/// #[derive(Debug, Default)]
/// struct RefMapOverride(DefaultReferenceMap);
/// impl CustomReferenceMap for RefMapOverride {
///     fn insert(&mut self, label: String, destination: String, title: Option<String>) -> bool {
///         self.0.insert(label, destination, title)
///     }
///
///     fn get(&self, label: &str) -> Option<(&str, Option<&str>)> {
///         // override a specific link
///         if label == "rust" {
///             return Some((
///                 "https://www.rust-lang.org/",
///                 Some("The Rust Language"),
///             ));
///         }
///         self.0.get(label)
///     }
/// }
///
/// struct AddCustomReferences;
/// impl CoreRule for AddCustomReferences {
///     fn run(root: &mut Node, _: &MarkdownThat) {
///         let data = root.cast_mut::<Root>().unwrap();
///         data.ext.insert(ReferenceMap::new(RefMapOverride::default()));
///     }
/// }
///
/// md.add_rule::<AddCustomReferences>()
///     .before::<BlockParserRule>();
///
/// let html = md.parse("[rust]").render();
/// assert_eq!(
///     html.trim(),
///     r#"<p><a href="https://www.rust-lang.org/" title="The Rust Language">rust</a></p>"#
/// );
/// ```
///
/// You can also view all references that the user created by adding the following rule:
///
/// ```rust
/// use markdown_that::parser::core::{CoreRule, Root};
/// use markdown_that::plugins::cmark::block::reference::{ReferenceMap, DefaultReferenceMap};
/// use markdown_that::{MarkdownThat, Node};
///
/// let md = &mut MarkdownThat::new();
/// markdown_that::plugins::cmark::add(md);
///
/// let ast = md.parse("[hello]: world");
/// let root = ast.node_value.downcast_ref::<Root>().unwrap();
/// let refmap = root.ext.get::<ReferenceMap>()
///     .map(|m| m.downcast_ref::<DefaultReferenceMap>().expect("expect references to be handled by default map"));
///
/// let mut labels = vec![];
/// if let Some(refmap) = refmap {
///     for (label, _dest, _title) in refmap.iter() {
///         labels.push(label);
///     }
/// }
///
/// assert_eq!(labels, ["hello"]);
/// ```
///
#[derive(Debug)]
pub struct ReferenceMap(Box<dyn CustomReferenceMap>);

impl Deref for ReferenceMap {
    type Target = Box<dyn CustomReferenceMap>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ReferenceMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl ReferenceMap {
    pub fn new(custom_map: impl CustomReferenceMap + 'static) -> Self {
        Self(Box::new(custom_map))
    }
}

impl Default for ReferenceMap {
    fn default() -> Self {
        Self::new(DefaultReferenceMap::new())
    }
}

impl RootExt for ReferenceMap {}

pub trait CustomReferenceMap: Debug + Downcast + Send + Sync {
    /// Insert a new element to the reference map. You may return false if it's not a valid label to stop parsing.
    fn insert(&mut self, label: String, destination: String, title: Option<String>) -> bool;

    /// Get an element referenced by `label` from the map, returns destination and optional title.
    fn get(&self, label: &str) -> Option<(&str, Option<&str>)>;
}

impl_downcast!(CustomReferenceMap);

#[derive(Default, Debug)]
pub struct DefaultReferenceMap(HashMap<ReferenceMapKey, ReferenceMapEntry>);

impl DefaultReferenceMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &str, Option<&str>)> {
        Box::new(
            self.0
                .iter()
                .map(|(a, b)| (a.label.as_str(), b.destination.as_str(), b.title.as_deref())),
        )
    }
}

impl CustomReferenceMap for DefaultReferenceMap {
    fn insert(&mut self, label: String, destination: String, title: Option<String>) -> bool {
        let Some(key) = ReferenceMapKey::new(label) else {
            return false;
        };
        self.0
            .entry(key)
            .or_insert(ReferenceMapEntry::new(destination, title));
        true
    }

    fn get(&self, label: &str) -> Option<(&str, Option<&str>)> {
        let key = ReferenceMapKey::new(label.to_owned())?;
        self.0
            .get(&key)
            .map(|r| (r.destination.as_str(), r.title.as_deref()))
    }
}

#[derive(Debug, Default, Educe, Eq)]
#[educe(Hash, PartialEq)]
/// Reference label
struct ReferenceMapKey {
    #[educe(PartialEq(ignore), Hash(ignore))]
    pub label: String,
    normalized: String,
}

impl ReferenceMapKey {
    pub fn new(label: String) -> Option<Self> {
        let normalized = normalize_reference(&label);

        if normalized.is_empty() {
            // CommonMark 0.20 disallows empty labels
            return None;
        }

        Some(Self { label, normalized })
    }
}

#[derive(Debug, Default)]
/// Reference value
struct ReferenceMapEntry {
    pub destination: String,
    pub title: Option<String>,
}

impl ReferenceMapEntry {
    pub fn new(destination: String, title: Option<String>) -> Self {
        Self { destination, title }
    }
}

/// Add a plugin that parses Markdown link references
pub fn add(md: &mut MarkdownThat) {
    md.block.add_rule::<ReferenceScanner>();
}

#[derive(Debug)]
pub struct Definition {
    pub label: String,
    pub destination: String,
    pub title: Option<String>,
}
impl NodeValue for Definition {
    fn render(&self, _: &Node, _: &mut dyn crate::Renderer) {}
}

#[doc(hidden)]
pub struct ReferenceScanner;
impl BlockRule for ReferenceScanner {
    fn check(_: &mut BlockState) -> Option<()> {
        None // can't interrupt anything
    }

    fn run(state: &mut BlockState) -> Option<(Node, usize)> {
        if state.line_indent(state.line) >= state.md.max_indent {
            return None;
        }

        let mut chars = state.get_line(state.line).chars();

        let Some('[') = chars.next() else {
            return None;
        };

        // Simple check to quickly interrupt the scan on [link](url) at the start of the line.
        // Can be useful in practice: https://github.com/markdown-it/markdown-it/issues/54
        loop {
            match chars.next() {
                Some('\\') => {
                    chars.next();
                }
                Some(']') => {
                    if let Some(':') = chars.next() {
                        break;
                    } else {
                        return None;
                    }
                }
                Some(_) => {}
                None => break,
            }
        }

        let start_line = state.line;
        let mut next_line = start_line;

        // jump line-by-line until empty one or EOF
        'outer: loop {
            next_line += 1;

            if next_line >= state.line_max || state.is_empty(next_line) {
                break;
            }

            // this may be a code block normally, but after a paragraph
            // it's considered a lazy continuation regardless of what's there
            if state.line_indent(next_line) >= state.md.max_indent {
                continue;
            }

            // quirk for blockquotes, that rule should already check this line
            if state.line_offsets[next_line].indent_nonspace < 0 {
                continue;
            }

            // Some tags can terminate a paragraph without an empty line.
            let old_state_line = state.line;
            state.line = next_line;
            if state.test_rules_at_line() {
                state.line = old_state_line;
                break 'outer;
            }
            state.line = old_state_line;
        }

        let (str_before_trim, _) = state.get_lines(start_line, next_line, state.blk_indent, false);
        let str = str_before_trim.trim();
        let mut chars = str.char_indices();
        chars.next(); // skip '['
        let label_end;
        let mut lines = 0;

        loop {
            match chars.next() {
                Some((_, '[')) => return None,
                Some((p, ']')) => {
                    label_end = p;
                    break;
                }
                Some((_, '\n')) => lines += 1,
                Some((_, '\\')) => {
                    if let Some((_, '\n')) = chars.next() {
                        lines += 1;
                    }
                }
                Some(_) => {}
                None => return None,
            }
        }

        let Some((_, ':')) = chars.next() else {
            return None;
        };

        // [label]:   destination   'title'
        //         ^^^ skip optional whitespace here
        let mut pos = label_end + 2;
        while let Some((_, ch @ (' ' | '\t' | '\n'))) = chars.next() {
            if ch == '\n' {
                lines += 1;
            }
            pos += 1;
        }

        // [label]:   destination   'title'
        //            ^^^^^^^^^^^ parse this
        let href;
        if let Some(res) = full_link::parse_link_destination(str, pos, str.len()) {
            if pos == res.pos {
                return None;
            }
            href = state.md.link_formatter.normalize_link(&res.str);
            state.md.link_formatter.validate_link(&href)?;
            pos = res.pos;
            lines += res.lines;
        } else {
            return None;
        }

        // to save the cursor state, we could require rolling back later
        let dest_end_pos = pos;
        let dest_end_lines = lines;

        // [label]:   destination   'title'
        //                       ^^^ skipping those spaces
        let start = pos;
        let mut chars = str[pos..].chars();
        while let Some(ch @ (' ' | '\t' | '\n')) = chars.next() {
            if ch == '\n' {
                lines += 1;
            }
            pos += 1;
        }

        // [label]:   destination   'title'
        //                          ^^^^^^^ parse this
        let mut title = None;
        if pos != start {
            if let Some(res) = full_link::parse_link_title(str, pos, str.len()) {
                title = Some(res.str);
                pos = res.pos;
                lines += res.lines;
            } else {
                pos = dest_end_pos;
                lines = dest_end_lines;
            }
        }

        // skip trailing spaces until the rest of the line
        let mut chars = str[pos..].chars();
        loop {
            match chars.next() {
                Some(' ' | '\t') => pos += 1,
                Some('\n') | None => break,
                Some(_) if title.is_some() => {
                    // garbage at the end of the line after the title,
                    // but it could still be a valid reference if we roll back
                    title = None;
                    pos = dest_end_pos;
                    lines = dest_end_lines;
                    chars = str[pos..].chars();
                }
                Some(_) => {
                    // garbage at the end of the line
                    return None;
                }
            }
        }

        let references = state.root_ext.get_or_insert_default::<ReferenceMap>();
        if !references.insert(str[1..label_end].to_owned(), href.clone(), title.clone()) {
            return None;
        }

        Some((
            Node::new(Definition {
                label: str[1..label_end].to_owned(),
                destination: href,
                title,
            }),
            lines + 1,
        ))
    }
}
