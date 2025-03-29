use toy_web_render::dom::{ elem, parse, text, Node, NodeType};
use std::collections::HashMap;

#[test]
fn test_parser_element_with_attributes() {
    let input = "<div class=\"container\" id=\"main\"></div>";
    let node = parse(input);

    match node.node_type {
        NodeType::Element(data) => {
            assert_eq!(data.tag_name, "div");
            assert_eq!(data.attributes.get("class").unwrap(), "container");
            assert_eq!(data.attributes.get("id").unwrap(), "main");
        }
        _ => panic!("Expected Element"),
    }
}

#[test]
fn test_parser_nested_elements() {
    let input = "<div><p>Hello</p></div>";
    let node = parse(input);

    match node.node_type {
        NodeType::Element(data) => {
            assert_eq!(data.tag_name, "div");
            assert_eq!(node.children.len(), 1);

            match &node.children[0].node_type {
                NodeType::Element(child_data) => {
                    assert_eq!(child_data.tag_name, "p");
                }
                _ => panic!("Expected Element for child"),
            }
        }
        _ => panic!("Expected Element"),
    }
}

#[test]
fn test_parser_comment() {
    let input = "<!-- test comment --><div></div>";
    let node = parse(input);

    assert_eq!(node.children.len(), 2);
    match &node.children[0].node_type {
        NodeType::Comment(content) => {
            assert_eq!(content, " test comment ");
        }
        _ => panic!("Expected Comment"),
    }
}

#[test]
fn test_parser_malformed_input() {
    let input = "<div><p>Hello</div>";
    let node = parse(input);
    // Should handle malformed HTML gracefully
    // Exact behavior depends on your parser implementation
    assert!(matches!(node.node_type, NodeType::Element(_)));
}
fn test_parse_empty() {
    let node = parse("");
    match node.node_type {
        NodeType::Element(data) => {
            assert_eq!(data.tag_name, "root");
            assert!(data.attributes.is_empty());
        }
        _ => panic!("Expected Element"),
    }
    assert!(node.children.is_empty());
}

#[test]
fn test_parse_text() {
    let node = text("Hello".to_string());
    match node.node_type {
        NodeType::Text(content) => assert_eq!(content, "Hello"),
        _ => panic!("Expected Text node"),
    }
    assert!(node.children.is_empty());
}

#[test]
fn test_create_element() {
    let mut attrs = HashMap::new();
    attrs.insert("class".to_string(), "test".to_string());

    let node = elem("div".to_string(), attrs, vec![text("content".to_string())]);

    match node.node_type {
        NodeType::Element(data) => {
            assert_eq!(data.tag_name, "div");
            assert_eq!(data.attributes.get("class").unwrap(), "test");
        }
        _ => panic!("Expected Element"),
    }

    assert_eq!(node.children.len(), 1);
}
