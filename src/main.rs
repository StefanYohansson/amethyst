#[macro_use]
extern crate nom;

pub mod dom;
pub mod html;

use std::iter;

fn inspect_dom(document: dom::Node) {
    fn iter_childrens(root_node: dom::Node, depth: usize) {
        let depth_space: String = iter::repeat(" ").take(depth).collect();
        for child in root_node.children {
            print!("{} |- {}", depth_space, child);
            if child.children.len() > 0 {
                iter_childrens(child, depth + 1);
            }
        }
    }

    print!("|_ {}", document);
    iter_childrens(document.clone(), 0);
}

fn parse_html() -> dom::Node {
    return html::parse("<html> <div>oi</div> </html>".to_string());
}

fn main() {
    inspect_dom(parse_html());
}
