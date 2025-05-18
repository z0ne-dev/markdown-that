use once_cell::sync::Lazy;


#[test]
fn title_example() {
    let parser = &mut markdown_that::MarkdownThat::new();
    markdown_that::plugins::cmark::add(parser);

    let ast = parser.parse("Hello **world**!");
    let html = ast.render();

    assert_eq!(html, "<p>Hello <strong>world</strong>!</p>\n");
}

#[test]
fn lazy_singleton() {
    static MD : Lazy<markdown_that::MarkdownThat> = Lazy::new(|| {
        let mut parser = markdown_that::MarkdownThat::new();
        markdown_that::plugins::cmark::add(&mut parser);
        parser
    });

    let ast = MD.parse("Hello **world**!");
    let html = ast.render();

    assert_eq!(html, "<p>Hello <strong>world</strong>!</p>\n");
}

#[test]
fn no_plugins() {
    let md = &mut markdown_that::MarkdownThat::new();
    let node = md.parse("hello\nworld");
    let result = node.render();
    assert_eq!(result, "hello\nworld\n");
}

#[test]
fn no_max_indent() {
    let md = &mut markdown_that::MarkdownThat::new();
    markdown_that::plugins::cmark::block::paragraph::add(md);
    markdown_that::plugins::cmark::block::list::add(md);
    md.max_indent = i32::MAX;
    let node = md.parse("        paragraph\n      - item");
    let result = node.render();
    assert_eq!(result, "<p>paragraph</p>\n<ul>\n<li>item</li>\n</ul>\n");
}


/*#[test]
fn no_block_parser() {
    let md = &mut markdown_that::MarkdownThat::new();
    markdown_that::plugins::cmark::add(md);
    md.remove_rule::<markdown_that::parser::block::builtin::BlockParserRule>();
    let node = md.parse("hello *world*");
    let result = node.render();
    assert_eq!(result, "hello <em>world</em>");
}*/

fn run(input: &str, output: &str) {
    let output = if output.is_empty() { "".to_owned() } else { output.to_owned() + "\n" };
    let md = &mut markdown_that::MarkdownThat::new();
    markdown_that::plugins::cmark::add(md);
    markdown_that::plugins::html::add(md);
    markdown_that::plugins::extra::beautify_links::add(md);
    let node = md.parse(&(input.to_owned() + "\n"));
    node.walk(|node, _| assert!(node.srcmap.is_some()));
    let result = node.render();
    assert_eq!(result, output);
}

mod markdown_that_rs_extras {
    use super::run;

    #[test]
    fn regression_test_img() {
        // ! at end of line
        run("Hello!", "<p>Hello!</p>");
    }

    #[test]
    fn regression_list_markers() {
        run("- foo\n- bar", "<ul>\n<li>foo</li>\n<li>bar</li>\n</ul>");
        run("1. foo\n1. bar", "<ol>\n<li>foo</li>\n<li>bar</li>\n</ol>");
    }

    #[test]
    fn tab_offset_in_lists() {
        run("   > -\tfoo\n   >\n   >         foo\n",
r#"<blockquote>
<ul>
<li>
<p>foo</p>
<pre><code> foo
</code></pre>
</li>
</ul>
</blockquote>"#);
    }

    #[test]
    fn null_char_replacement() {
        run("&#0;", "<p>\u{FFFD}</p>");
        run("\0", "<p>\u{FFFD}</p>");
    }

    #[test]
    fn cr_only_newlines() {
        run("foo\rbar", "<p>foo\nbar</p>");
        run("    foo\r    bar", "<pre><code>foo\nbar\n</code></pre>");
    }

    #[test]
    fn cr_lf_newlines() {
        run("foo\r\nbar", "<p>foo\nbar</p>");
        run("    foo\r\n    bar", "<pre><code>foo\nbar\n</code></pre>");
    }

    #[test]
    fn beautify_links() {
        run("<https://www.reddit.com/r/programming/comments/vxttiq/comment/ifyqsqt/?utm_source=reddit&utm_medium=web2x&context=3>",
            "<p><a href=\"https://www.reddit.com/r/programming/comments/vxttiq/comment/ifyqsqt/?utm_source=reddit&amp;utm_medium=web2x&amp;context=3\">www.reddit.com/r/programming/comments/…/ifyqsqt/?…</a></p>");
    }

    #[test]
    fn regression_test_newlines_with_images() {
        run("There is a newline in this image  ![here\nit is](https://github.com/executablebooks/)",
            "<p>There is a newline in this image  <img src=\"https://github.com/executablebooks/\" alt=\"here\nit is\"></p>");
    }

    #[test]
    fn test_node_ext_propagation() {
        use markdown_that::parser::block::{BlockRule, BlockState};
        use markdown_that::parser::core::CoreRule;
        use markdown_that::parser::extset::NodeExt;
        use markdown_that::parser::inline::{InlineRule, InlineState};
        use markdown_that::{MarkdownThat, Node};

        #[derive(Debug, Default)]
        struct NodeErrors(Vec<&'static str>);
        impl NodeExt for NodeErrors {}

        struct MyInlineRule;
        impl InlineRule for MyInlineRule {
            const MARKER: char = '@';

            fn run(state: &mut InlineState) -> Option<(Node, usize)> {
                let err = state.node.ext.get_or_insert_default::<NodeErrors>();
                err.0.push("inline");
                None
            }
        }

        struct MyBlockRule;
        impl BlockRule for MyBlockRule {
            fn run(state: &mut BlockState) -> Option<(Node, usize)> {
                let err = state.node.ext.get_or_insert_default::<NodeErrors>();
                err.0.push("block");
                None
            }
        }

        struct MyCoreRule;
        impl CoreRule for MyCoreRule {
            fn run(root: &mut Node, _md: &MarkdownThat) {
                let err = root.ext.get_or_insert_default::<NodeErrors>();
                err.0.push("core");
            }
        }

        let md = &mut markdown_that::MarkdownThat::new();
        markdown_that::plugins::cmark::add(md);

        md.inline.add_rule::<MyInlineRule>();
        md.block.add_rule::<MyBlockRule>();
        md.add_rule::<MyCoreRule>().after_all();

        let text1 = r#"*hello @world*"#;
        let ast = md.parse(text1);
        let mut collected: Vec<&str> = vec![];

        ast.walk_post(|node, _| {
            if let Some(errors) = node.ext.get::<NodeErrors>() {
                collected.extend(errors.0.iter());
            }
        });

        assert_eq!(
            collected,
            vec!["inline", "block", "core"],
        );
    }
}

mod examples {
    include!("../examples/ferris/main.rs");

    #[test]
    fn test_examples() {
        main();
    }
}

