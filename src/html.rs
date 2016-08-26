use dom;
use std::collections::HashMap;

pub fn parse(source: String) -> dom::Node {
    let mut nodes = Parser { pos: 0, input: source }.parse_nodes();

    // If the document contains a root element, just return it. Otherwise, create one.
    if nodes.len() == 1 {
        nodes.swap_remove(0)
    } else {
        dom::el("html".to_string(), HashMap::new(), nodes)
    }
}

struct Parser {
    pos: usize,
    input: String
}

impl Parser {
    fn parse_nodes(&mut self) -> Vec<dom::Node> {
        let input = self.input.clone();
        self.parse_node(input);
        return vec!(dom::fixture_node(), dom::fixture_node());
    }

    fn parse_node(&mut self, slice: String) {
        named!(tags, delimited!(char!('<'), is_not!(">"), char!('>')));
        let bytes = slice.bytes().collect::<Vec<u8>>();
        println!( "{:?}", tags( &bytes ));
    }
}
