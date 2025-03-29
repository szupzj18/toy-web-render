use std::collections::HashMap;

#[derive(Debug)]
pub struct Node { // a dom node
    pub node_type: NodeType,
    pub children: Vec<Node>,
}

#[derive(Debug)]
pub struct ElementData {
    pub tag_name: String,
    pub attributes: AttrMap, // (name, value)
}

type AttrMap = HashMap<String, String>;

#[derive(Debug)]
pub enum NodeType {
    Element(ElementData),
    Text(String),
    Comment(String),
}

// create text node
pub fn text(data: String) -> Node {
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

pub struct Parser {
    pos: usize,
    input: String,
}

impl Parser {

    fn eof(&self) -> bool {
        self.pos >= self.input.len()
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