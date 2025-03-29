pub mod dom;
pub mod css;

use std::fs::File;
use std::io::Read;

fn main() {
    // 1. read html file from example/test.html
    let mut file = File::open("example/test.html").expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file");
    println!("File contents:\n {}", contents);
    // 2. parse html file
    let dom = dom::parse(&contents);
    // 3. print dom
    println!("DOM: {:?}", dom);
}
