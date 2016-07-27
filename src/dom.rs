//! DOM data structure

use std::collections::{HashMap, HashSet};
use std::fmt;

pub type AttrMap = HashMap<String, String>;

#[derive(Debug)]
pub struct Node {
    pub children: Vec<Node>,
    pub node_type: NodeType
}

#[derive(Debug)]
pub enum NodeType {
    Text(String),
    Element(ElementData)
}

#[derive(Debug)]
pub struct ElementData {
    tag_name: String,
    attributes: AttrMap
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.node_type {
            &NodeType::Text(ref x) => writeln!(f, "Node::Text -> {}", x),
            &NodeType::Element(ref elem) => writeln!(f, "Node::Element -> #{}",
                                                     elem.id().unwrap())
        }
    }
}

// implementation for element struct
impl ElementData {
    pub fn id(&self) -> Option<&String> {
        self.attributes.get("id")
    }

    pub fn classes(&self) -> HashSet<&str> {
        match self.attributes.get("class") {
            Some(classlist) => classlist.split(' ').collect(),
            None => HashSet::new()
        }
    }
}

// convention fn for text creation
pub fn text(data: String) -> Node {
    Node { children: vec![], node_type: NodeType::Text(data) }
}

// convention fn for element creation
pub fn el(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
    Node {
        children: children,
        node_type: NodeType::Element(ElementData {
            tag_name: name,
            attributes: attrs,
        })
    }
}

// fixture node for test purpose
pub fn fixture_node() -> Node {
    let mut attrs = HashMap::new();
    attrs.insert("class".to_string(), "btn btn-test".to_string());
    attrs.insert("id".to_string(), "close".to_string());
    
    let mut childrens = Vec::new();
    let text:Node = text("bfc".to_string());
    childrens.push(text);
    
    return el("div".to_string(), attrs, childrens);
}

#[test]
fn it_create_el() {
    let el:Node = fixture_node();

    assert_eq!(el.children.len(), 1);

    match el.node_type {
        NodeType::Element(ref elem) => {
            assert_eq!(elem.id(), Some(&"close".to_string()));
            assert_eq!(elem.classes().len(), 2);
        },
        NodeType::Text(_) => {}
    }
}