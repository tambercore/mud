use crate::ccg::ccg_types::CCGNode;
use crate::ccg::language_parser::english_to_ccg;

fn ccg_to_fol(ccg: CCGNode) {
    print_nodes_with_text(&ccg);
}

/// Traverses the CCGNode and prints out nodes that have a "text" value.
fn print_nodes_with_text(node: &CCGNode) {
    // If the node has a text value, print it.
    if let Some(ref text) = node.text {
        println!("Terminal node: {}", node);
    }

    // Recursively traverse the child nodes if they exist.
    if let Some(ref children) = node.children {
        for child in children {
            print_nodes_with_text(child); // Recursively print for each child.
        }
    }
}


#[test]
fn run() {
    let sentence = "John likes cheese"; // Likes(John, Cheese)
    let ccg = english_to_ccg(sentence).expect("Failed to parse sentence to CCG.");
    println!("{}", ccg);
    // let fol = ccg_to_fol(ccg);
}