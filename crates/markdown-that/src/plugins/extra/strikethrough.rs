//! Strikethrough syntax (like `~~this~~`)
use crate::generics::inline::emph_pair;
use crate::{MarkdownThat, Node, NodeValue, Renderer};

#[derive(Debug)]
pub struct Strikethrough {
    pub marker: char,
}

impl NodeValue for Strikethrough {
    fn render(&self, node: &Node, fmt: &mut dyn Renderer) {
        fmt.open("s", &node.attrs);
        fmt.contents(&node.children);
        fmt.close("s");
    }
}

pub fn add(md: &mut MarkdownThat) {
    emph_pair::add_with::<'~', 2, true>(md, || Node::new(Strikethrough { marker: '~' }));
}
