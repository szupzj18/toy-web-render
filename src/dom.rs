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
    fn consume_next_char(&mut self) -> Option<char> {
        let c = self.next_char()?;
        // move forward by one character
        self.pos += c.len_utf8(); // move the position forward
        Some(c)
    }
    /// slice the input string
    /// from pos to the end
    fn next_char(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    /// use starts with to check if the input string contains specific token
    fn starts_with(&self, prefix: &str) -> bool {
        self.input[self.pos..].starts_with(prefix)
    }

    fn expect(&mut self, s: &str) {
        if self.starts_with(s) {
            self.pos += s.len(); // scanner moves forward
            // consume the token
        } else {
            // panic if the token is not found
            panic!("Expected {} at {}, found {}", s, self.pos, self.next_char().unwrap_or('\0'));
        }
    }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_char() {
        let parser = Parser {
            pos: 0,
            input: String::from("Hello"),
        };

        assert_eq!(parser.next_char(), Some('H'));
    }

    #[test]
    fn test_next_char_empty() {
        let parser = Parser {
            pos: 0,
            input: String::new(),
        };

        assert_eq!(parser.next_char(), None);
    }

    #[test]
    fn test_next_char_at_end() {
        let parser = Parser {
            pos: 5,
            input: String::from("Hello"),
        };

        assert_eq!(parser.next_char(), None);
    }

    #[test]
    fn test_starts_with() {
        let parser = Parser {
            pos: 0,
            input: String::from("<div>Hello</div>"),
        };

        assert!(parser.starts_with("<div>"));
        assert!(!parser.starts_with("<span>"));
    }

    #[test]
    #[should_panic(expected = "Expected <span> at 0")]
    fn test_expect_failure() {
        let mut parser = Parser {
            pos: 0,
            input: String::from("<div>Hello</div>"),
        };

        parser.expect("<span>");
    }

    #[test]
    fn test_eof() {
        let parser = Parser {
            pos: 16,
            input: String::from("<div>Hello</div>"),
        };

        assert!(parser.eof());
    }
    
    #[test]
    fn test_consume_next_char() {
        let mut parser = Parser {
            pos: 0,
            input: String::from("Hello"),
        };
        
        assert_eq!(parser.consume_next_char(), Some('H'));
        assert_eq!(parser.pos, 1); // Position advanced
    }

    #[test]
    fn test_consume_next_char_empty() {
        let mut parser = Parser {
            pos: 0,
            input: String::new(),
        };
        
        assert_eq!(parser.consume_next_char(), None);
    }
}