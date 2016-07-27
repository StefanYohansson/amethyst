pub mod dom;

fn inspect_dom(document:dom::Node) {
    fn iter_childrens(root_node: &dom::Node) {
        for child in &root_node.children {
            println!("{}", child);
            if child.children.len() > 0 {
                iter_childrens(&child);
            }
        }
    }
    
    iter_childrens(&document);
}

fn main() {
    let el:dom::Node = dom::fixture_node();
    inspect_dom(el);
}
