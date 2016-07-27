pub mod dom;

use std::iter;

fn inspect_dom(document:dom::Node) {
    fn iter_childrens(root_node: &dom::Node, depth: usize) {
        print!("|_ {}", root_node);
        let depth_space: String = iter::repeat(" ").take(depth).collect();
        for child in &root_node.children {
            print!("{} |- {}", depth_space, child);
            if child.children.len() > 0 {
                iter_childrens(&child, depth + 1);
            }
        }
    }
    
    iter_childrens(&document, 0);
}

fn main() {
    let el:dom::Node = dom::fixture_node();
    inspect_dom(el);
}
