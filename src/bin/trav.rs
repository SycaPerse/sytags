extern crate rctree;

use html5ever::parse_document;
use html5ever::rcdom::{Handle, RcDom};
use html5ever::tendril::TendrilSink;
use rctree::Node;
use std::fs::File;
use std::io::Read;

fn print_node(indent: usize, handle: Handle) {
    let node = handle.borrow();
    let attrs = node.attributes.borrow();

    // Print the node name and any attributes
    print!("{:indent$}", "", indent = indent);
    print!("{} ", node.name.local);
    for attr in attrs.iter() {
        print!("({}=\"{}\") ", attr.name.local, attr.value);
    }
    println!();

    // Recursively print child nodes
    for child in node.children.iter() {
        print_node(indent + 2, child.clone());
    }
}

fn main() {
    let mut html_file = File::open("path_to_your_html_file").unwrap();
    let mut html = String::new();
    html_file.read_to_string(&mut html).unwrap();

    let dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut html.as_bytes())
        .unwrap();

    print_node(0, dom.document);
}
