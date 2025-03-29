/// style sheet is a bunch of rules
/// that are applied to elements
/// this css parser is a simple one
/// it only supports a subset of css
struct Stylesheet {
    rules: Vec<Rule>,
}

enum Selector {
    Simple(SimpleSelector),
}

struct SimpleSelector {
    tag_name: Option<String>,
    id: Option<String>,
    class: Vec<String>,
}
struct Rule {
    selectors: Vec<Selector>,
    declarations: Vec<Declaration>,
}

/// A declaration is a property-value pair
/// e.g. color: red;
/// it is a part of a rule
/// e.g. h1 { color: red; }
/// e.g. h1 { margin: auto; }
struct Declaration {
    property: String,
    value: String,
}

enum Value {
    Keyword(String),
    Length(f32, Unit),
    ColorValue(color),
    // add more
}

enum Unit {
    Px,
    // add more
}

struct color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}