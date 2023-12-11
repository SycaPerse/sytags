use std::collections::HashMap;

// use libxml::parser::Parser;
use libxml::tree::*;

fn format_attribute(key: &str, value: &str) -> String {
    format!("{}=\"{}\"", key, value)
}

fn attr_lists(attr: HashMap<String, String>) -> String {
    let mut result = Vec::new();
    for (k, v) in attr {
        result.push(format_attribute(&k, &v));
    }
    result.join(", ")
}

pub fn traverse(node: &Node, depth: usize) {
    let indent = " ".repeat(depth * 2);
    if node.get_type().unwrap() == NodeType::ElementNode {
        let attr = node.get_properties();
        let attr_list = attr_lists(attr);
        println!("{}{}({}) {{", indent, node.get_name(), attr_list);
    }

    let content = node.get_content();

    let cleaned_text = content.split_whitespace().collect::<Vec<&str>>().join(" ");

    let mut c: Option<Node> = node.get_first_child();
    while let Some(child) = c {
        traverse(&child, depth + 1);
        c = child.get_next_sibling();
    }

    if !cleaned_text.is_empty() {
        // check if the content is not empty
        println!("\"{}\"", cleaned_text); // there is a little bug on the text. which keeps litering around
    }
    if node.get_type().unwrap() == NodeType::ElementNode {
        println!("{}}}", indent);
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = myadd(2, 2);
//         assert_eq!(result, 4);
//     }
// }
