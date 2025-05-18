use crate::parser::core::{CoreRule, Root};
use crate::{MarkdownThat, Node};

pub fn add(md: &mut MarkdownThat) {
    md.add_rule::<BlockParserRule>().before_all();
}

pub struct BlockParserRule;
impl CoreRule for BlockParserRule {
    fn run(root: &mut Node, md: &MarkdownThat) {
        let mut node = std::mem::take(root);
        let data = node.cast_mut::<Root>().unwrap();
        let source = std::mem::take(&mut data.content);
        let mut ext = std::mem::take(&mut data.ext);

        node = md.block.parse(source.as_str(), node, md, &mut ext);
        let data = node.cast_mut::<Root>().unwrap();
        data.content = source;
        data.ext = ext;
        *root = node;
    }
}
