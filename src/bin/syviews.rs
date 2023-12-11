use std::collections::HashMap;

use libxml::parser::Parser;
use libxml::tree::*;

// define a struct to represent a tree node
#[derive(Debug)]
struct TreeNode {
    tag: String,
    attrs: Vec<HashMap<String, String>>,
    text: String,
    children: Vec<TreeNode>,
}
//function format attribute, return ("%s=&s", key, value)
fn format_attribute(key: &str, value: &str) -> String {
    format!("{}=\"{}\"", key, value)
}

// define a function to create a tree node from a hashmap
fn create_tree_node(data: TreeNode) -> TreeNode {
    // get the tag and the children from the hashmap
    let (tag, children) = data.into_iter().next().unwrap();
    // create a tree node with the tag
    let mut node = TreeNode {
        tag: tag.to_string(),
        children: Vec::new(),
    };
    // recursively create the children nodes and append them to the node
    for child in children {
        node.children.push(create_tree_node(child));
    }
    // return the node
    node
}

// define a function to generate a template tag from a tree node
fn template_tags(node: &TreeNode, c_tag: &str) -> Vec<String> {
    // get the tag from the node
    let d_tag = &node.tag;
    // create a template tag with the current tag and the parent tag
    let tag_tree = format!("{} > {}", c_tag, d_tag);
    // create a vector to store the template tags
    let mut result = vec![tag_tree];
    // iterate over the children nodes and recursively generate the template tags
    for child in &node.children {
        // get the child tag
        let c_tag = &child.tag;
        // get the template tags from the child node
        let res = template_tags(child, c_tag);
        // extend the result vector with the template tags from the child node
        result.extend(res.iter().map(|s| format!("    {}", s)));
    }
    // return the result vector
    result
}

fn attr_lists(attr: HashMap<String, String>) -> String {
    let mut result = Vec::new();
    for (k, v) in attr {
        result.push(format_attribute(&k, &v));
    }
    result.join(", ")
}

fn my_recurse(node: &Node) {
    if node.get_type().unwrap() == NodeType::ElementNode {
        let attr = node.get_properties();
        let attr_list = attr_lists(attr);
        let content = node
            .get_content()
            .trim_matches(|c| c == '+' || c == '\n' || c == ' ' || c == '\t')
            .to_string();
        println!("{} {{", node.get_name());
        println!("({}) {{", attr_list);
        if !content.is_empty() {
            // check if the content is not empty
            println!("++{}++", content); // print the content
        }
        println!("}}");
    }
    // println!("}}");

    let mut c: Option<Node> = node.get_first_child();
    while let Some(child) = c {
        my_recurse(&child);
        c = child.get_next_sibling();
    }
}

fn main() {
    let parser = Parser::default();
    let doc = parser
        .parse_file("/home/afidegnum/Projects/Labs/Nvim/Syca/sytags/index.html")
        .unwrap();
    let root = doc.get_root_element().unwrap();
    my_recurse(&root);
}
