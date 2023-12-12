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
        .join(" ");
    println!("{}{}({}) {{", " ".repeat(indent), name, attrs); // Add an opening brace here

    for child in element.children() {
        match child.value() {
            scraper::Node::Element(_) => {
                let child_ref = ElementRef::wrap(child).unwrap();
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
