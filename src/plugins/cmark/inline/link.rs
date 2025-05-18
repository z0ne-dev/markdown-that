//! Links
//!
//! `![link](<to> "stuff")`
//!
//! <https://spec.commonmark.org/0.30/#links>
use crate::generics::inline::full_link;
use crate::{MarkdownThat, Node, NodeValue, Renderer};

#[derive(Debug)]
pub struct Link {
    pub url: String,
    pub title: Option<String>,
}

impl NodeValue for Link {
    fn render(&self, node: &Node, fmt: &mut dyn Renderer) {
        let mut attrs = node.attrs.clone();
        attrs.push(("href", self.url.clone()));

        if let Some(title) = &self.title {
            attrs.push(("title", title.clone()));
        }

        fmt.open("a", &attrs);
        fmt.contents(&node.children);
        fmt.close("a");
    }
}

pub fn add(md: &mut MarkdownThat) {
    full_link::add::<false>(md, |href, title| Node::new(Link {
        url: href.unwrap_or_default(),
        title,
    }));
}
