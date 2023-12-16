extern crate scraper;

use scraper::ElementRef;
use std::fs::File;
use std::io::Write;

fn format_attribute(key: &str, value: &str) -> String {
    format!("{}=\"{}\"", key, value)
}

pub fn write_tree(element: ElementRef, indent: usize, file: &mut File) {
    let name = element.value().name();
    let attrs = element
        .value()
        .attrs()
        .map(|(k, v)| format_attribute(k, v))
        .collect::<Vec<_>>()
        .join(", ");
    writeln!(file, "{}{}({}) {{", " ".repeat(indent), name, attrs).unwrap();

    let mut child = element.first_child();
    while let Some(node) = child {
        match node.value() {
            scraper::Node::Element(_) => {
                let child_ref = ElementRef::wrap(node).unwrap();
                write_tree(child_ref, indent + 2, file);
            }
            scraper::Node::Text(text_node) => {
                let trimmed_text = text_node.trim();
                if !trimmed_text.is_empty() {
                    let txt = format!("{}\"{}\"", " ".repeat(indent + 2), trimmed_text);
                    writeln!(file, "{}", txt).unwrap();
                }
            }
            _ => {}
        }
        child = node.next_sibling();
    }
    writeln!(file, "{}}}", " ".repeat(indent)).unwrap();
}

pub fn walk_tree(element: ElementRef, indent: usize) {
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
