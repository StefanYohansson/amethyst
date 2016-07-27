pub mod dom;

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

fn main() {
    let el: dom::Node = dom::fixture_node();
    inspect_dom(el);
}
