//! HTML inline syntax from CommonMark
//!
//! <https://spec.commonmark.org/0.30/#raw-html>
use super::utils::regexps::*;
use crate::parser::inline::{InlineRule, InlineState};
use crate::{MarkdownThat, Node, NodeValue, Renderer};

#[derive(Debug)]
pub struct HtmlInline {
    pub content: String,
}

impl NodeValue for HtmlInline {
    fn render(&self, _: &Node, fmt: &mut dyn Renderer) {
        fmt.text_raw(&self.content);
    }
}

pub fn add(md: &mut MarkdownThat) {
    md.inline.add_rule::<HtmlInlineScanner>();
}

#[doc(hidden)]
pub struct HtmlInlineScanner;
impl InlineRule for HtmlInlineScanner {
    const MARKER: char = '<';

    fn run(state: &mut InlineState) -> Option<(Node, usize)> {
        // Check start
        let mut chars = state.src[state.pos..state.pos_max].chars();
        if chars.next().unwrap() != '<' { return None; }

        // Quick fail on second char
        let Some('!' | '?' | '/' | 'A'..='Z' | 'a'..='z') = chars.next() else { return None; };

        let capture = HTML_TAG_RE.captures(&state.src[state.pos..state.pos_max])?.get(0).unwrap().as_str();
        let capture_len = capture.len();

        let content = capture.to_owned();

        if HTML_LINK_OPEN.is_match(&content) {
            state.link_level += 1;
        } else if HTML_LINK_CLOSE.is_match(&content) {
            state.link_level -= 1;
        }

        let node = Node::new(HtmlInline { content });
        Some((node, capture_len))
    }
}
