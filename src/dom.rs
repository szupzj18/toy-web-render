use std::collections::HashMap;
use std::fmt;

struct Node { // a dom node
    node_type: NodeType,
    children: Vec<Node>,
}

struct ElementData {
    tag_name: String,
    attributes: AttrMap, // (name, value)
}

type AttrMap = HashMap<String, String>;

enum NodeType {
    Element(ElementData),
    Text(String),
    Comment(String),
}

// create text node
fn text(data: String) -> Node {
    Node {
        node_type: NodeType::Text(data),
        children: Vec::new(),
    }
}

// create element node
pub fn elem(tag_name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
    Node {
        children,
        node_type: NodeType::Element(ElementData {
            tag_name,
            attributes: attrs,
        }),
    }
}