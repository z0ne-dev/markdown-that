use crate::MarkdownThat;

pub(super) mod block_parser;

pub use block_parser::BlockParserRule;

pub fn add(md: &mut MarkdownThat) {
    block_parser::add(md);
}
