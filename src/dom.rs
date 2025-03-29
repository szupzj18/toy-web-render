use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub struct Node { // a dom node
    node_type: NodeType,
    children: Vec<Node>,
}

#[derive(Debug)]
struct ElementData {
    tag_name: String,
    attributes: AttrMap, // (name, value)
}

type AttrMap = HashMap<String, String>;

#[derive(Debug)]
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

pub fn parse(content: &str) -> Node {
    // 1. create a root node
    let mut root = Node {
        node_type: NodeType::Element(ElementData {
            tag_name: "root".to_string(),
            attributes: HashMap::new(),
        }),
        children: Vec::new(),
    };
    // 2. parse the content and create nodes
    // 3. add nodes to the root
    // 4. return the root node
    root
}