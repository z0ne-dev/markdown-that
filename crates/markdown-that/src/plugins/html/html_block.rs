//! HTML block syntax from CommonMark
//!
//! <https://spec.commonmark.org/0.30/#html-blocks>

use regex::Regex;
use std::sync::LazyLock;

use super::utils::blocks::*;
use super::utils::regexps::*;
use crate::parser::block::{BlockRule, BlockState};
use crate::{MarkdownThat, Node, NodeValue, Renderer};

#[derive(Debug)]
pub struct HtmlBlock {
    pub content: String,
}

impl NodeValue for HtmlBlock {
    fn render(&self, _: &Node, fmt: &mut dyn Renderer) {
        fmt.cr();
        fmt.text_raw(&self.content);
        fmt.cr();
    }
}

pub fn add(md: &mut MarkdownThat) {
    md.block.add_rule::<HtmlBlockScanner>();
}

struct HTMLSequence {
    open: Regex,
    close: Regex,
    can_terminate_paragraph: bool,
}

impl HTMLSequence {
    pub fn new(open: Regex, close: Regex, can_terminate_paragraph: bool) -> Self {
        Self {
            open,
            close,
            can_terminate_paragraph,
        }
    }
}

// An array of opening and corresponding closing sequences for html tags,
// last argument defines whether it can terminate a paragraph or not
//
static HTML_SEQUENCES: LazyLock<[HTMLSequence; 7]> = LazyLock::new(|| {
    let block_names = HTML_BLOCKS.join("|");
    let open_close_tag_re = HTML_OPEN_CLOSE_TAG_RE.as_str();

    [
        HTMLSequence::new(
            Regex::new(r#"(?i)^<(script|pre|style|textarea)(\s|>|$)"#).unwrap(),
            Regex::new(r#"(?i)</(script|pre|style|textarea)>"#).unwrap(),
            true,
        ),
        HTMLSequence::new(
            Regex::new(r#"^<!--"#).unwrap(),
            Regex::new(r#"-->"#).unwrap(),
            true,
        ),
        HTMLSequence::new(
            Regex::new(r#"^<\?"#).unwrap(),
            Regex::new(r#"\?>"#).unwrap(),
            true,
        ),
        HTMLSequence::new(
            Regex::new(r#"^<![A-Z]"#).unwrap(),
            Regex::new(r#">"#).unwrap(),
            true,
        ),
        HTMLSequence::new(
            Regex::new(r#"^<!\[CDATA\["#).unwrap(),
            Regex::new(r#"\]\]>"#).unwrap(),
            true,
        ),
        HTMLSequence::new(
            Regex::new(&format!("(?i)^</?({block_names})(\\s|/?>|$)")).unwrap(),
            Regex::new(r#"^$"#).unwrap(),
            true,
        ),
        HTMLSequence::new(
            Regex::new(&format!("{open_close_tag_re}\\s*$")).unwrap(),
            Regex::new(r#"^$"#).unwrap(),
            false,
        ),
    ]
});

#[doc(hidden)]
pub struct HtmlBlockScanner;

impl HtmlBlockScanner {
    fn get_sequence(state: &mut BlockState) -> Option<&'static HTMLSequence> {
        if state.line_indent(state.line) >= state.md.max_indent {
            return None;
        }

        let line_text = state.get_line(state.line);
        let Some('<') = line_text.chars().next() else {
            return None;
        };

        let mut sequence = None;
        for seq in HTML_SEQUENCES.iter() {
            if seq.open.is_match(line_text) {
                sequence = Some(seq);
                break;
            }
        }

        sequence
    }
}

impl BlockRule for HtmlBlockScanner {
    fn check(state: &mut BlockState) -> Option<()> {
        let sequence = Self::get_sequence(state)?;
        if !sequence.can_terminate_paragraph {
            return None;
        }
        Some(())
    }

    fn run(state: &mut BlockState) -> Option<(Node, usize)> {
        let sequence = Self::get_sequence(state)?;

        let line_text = state.get_line(state.line);
        let start_line = state.line;
        let mut next_line = state.line + 1;

        // If we are here - we detected HTML block.
        // Let's roll down till block end.
        if !sequence.close.is_match(line_text) {
            while next_line < state.line_max {
                if state.line_indent(next_line) < 0 {
                    break;
                }

                let line_text = state.get_line(next_line);

                if sequence.close.is_match(line_text) {
                    if !line_text.is_empty() {
                        next_line += 1;
                    }
                    break;
                }

                next_line += 1;
            }
        }

        let (content, _) = state.get_lines(start_line, next_line, state.blk_indent, true);
        let node = Node::new(HtmlBlock { content });
        Some((node, next_line - state.line))
    }
}
