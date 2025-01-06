use serde::{Deserialize, Deserializer};
use serde::de::{self, MapAccess, Visitor};
use std::fmt;
use crate::ccg::word::CCGWord;
use super::rule::CCGRule;
use super::category::CCGType;
use ascii_tree::{Tree::*, Tree, write_tree};


#[derive(Debug, Clone, Deserialize)]
pub struct CCGNode {

    #[serde(rename = "type")]
    pub node_type: CCGType,

    #[serde(rename = "text")]
    pub word: Option<CCGWord>,

    pub rule: CCGRule,
    pub children: Option<Vec<Box<CCGNode>>>, // Use Box to handle recursion
}

impl CCGNode {
    /// Performs an in-order traversal of the CCGNode tree.
    /// Collects references to nodes in the provided vector in in-order sequence.
    pub fn inorder_traversal<'a>(&'a self, visit: &mut Vec<&'a CCGNode>) {
        if let Some(children) = &self.children {
            // Traverse left child if it exists
            if children.len() >= 1 {
                children[0].inorder_traversal(visit);
            }

            // Visit the current node
            visit.push(self);

            // Traverse right child if it exists
            if children.len() >= 2 {
                children[1].inorder_traversal(visit);
            }

            // If there are more than two children, you can decide how to handle them.
            // For standard binary in-order traversal, additional children are ignored or handled differently.
            // Here, we'll ignore them, but you can modify this behavior as needed.
        } else {
            // Leaf node, just visit
            visit.push(self);
        }
    }
}


impl fmt::Display for CCGNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn collect_words(node: &CCGNode) -> String {
            if let Some(word) = &node.word {
                word.text.clone() // Terminal node with a word
            } else if let Some(children) = &node.children {
                children
                    .iter()
                    .map(|child| collect_words(child))
                    .filter(|word| !word.is_empty())
                    .collect::<Vec<_>>()
                    .join(" ")
            } else {
                String::new() // No word and no children
            }
        }

        fn to_ascii_tree(node: &CCGNode) -> Tree {
            let aggregated_word = collect_words(node);
            let title = format!(
                "'{}', {}, [{}]",
                if aggregated_word.is_empty() {
                    "".to_string()
                } else {
                    aggregated_word
                },
                node.node_type,
                node.rule
            );

            let children = if let Some(children) = &node.children {
                children.iter().map(|child| to_ascii_tree(child)).collect()
            } else {
                vec![]
            };
            Node(title, children)
        }

        let ascii_tree = to_ascii_tree(self);
        let mut output = String::new();
        write_tree(&mut output, &ascii_tree).map_err(|_| fmt::Error)?;
        write!(f, "{}", output)
    }
}