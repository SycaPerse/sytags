extern crate scraper;

use scraper::{ElementRef, Html, Selector};
use std::fs;

fn format_attribute(key: &str, value: &str) -> String {
    format!("{}=\"{}\"", key, value)
}

fn walk_tree(element: ElementRef, indent: usize) {
    let name = element.value().name();
    let attrs = element
        .value()
        .attrs()
        .map(|(k, v)| format_attribute(k, v))
        .collect::<Vec<_>>()
        .join(", ");
    println!("{}{}({}) {{", " ".repeat(indent), name, attrs); // Add an opening brace here

    let mut child = element.first_child(); // Get the first child node
    while let Some(node) = child {
        // While there is a child node
        match node.value() {
            scraper::Node::Element(_) => {
                let child_ref = ElementRef::wrap(node).unwrap();
                walk_tree(child_ref, indent + 2);
            }
            scraper::Node::Text(text_node) => {
                let trimmed_text = text_node.trim();
                if !trimmed_text.is_empty() {
                    let txt = format!("{}\"{}\"", " ".repeat(indent + 2), trimmed_text);
                    println!("{}", txt); // Remove the closing brace here
                }
            }
            _ => {}
        }
        child = node.next_sibling(); // Update the child node to the next sibling
    }
    println!("{}}}", " ".repeat(indent)); // Add a closing brace here
}

fn main() {
    let html_file = fs::read_to_string("/home/afidegnum/Projects/Labs/Nvim/Syca/sytags/index.html")
        .expect("Unable to read file");
    let document = Html::parse_document(&html_file);
    let body = document.root_element();

    walk_tree(body, 0);
}
