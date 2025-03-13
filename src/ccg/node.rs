use std::cmp::PartialEq;
use serde::Deserialize;
use std::fmt;
use crate::ccg::word::CCGWord;
use super::rule::CCGRule;
use super::category::CCGType;
use ascii_tree::{Tree::*, Tree, write_tree};
use uuid::Uuid;



/// A structure to encapsulate a node of hierarchial information returned by `lambeq`.
/// In the CCG Tree, each `CCGNode` has a `CCGType` and `CCGRule`.
/// Terminal nodes are associated with words. Non-terminal nodes point to children.
#[derive(Debug, Clone, Deserialize)]
pub struct CCGNode {
    #[serde(rename = "type")]
    pub node_type: CCGType,

    #[serde(rename = "text")]
    pub word: Option<CCGWord>,

    pub rule: CCGRule,
    pub children: Option<Vec<Box<CCGNode>>>,

    /* A unique identifier to differentiate nodes with the same text. */
    #[serde(skip)]
    pub id: Uuid
}



/// Implementation of pretty print for `CCGNode`.
impl fmt::Display for CCGNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn collect_words(node: &CCGNode) -> String {
            /* Return the text of terminal nodes. */
            if let Some(word) = &node.word {
                word.text.clone()
            }

            /* For non-terminals, collect the text of their children. */
            else if let Some(children) = &node.children {
                children
                    .iter()
                    .map(|child| collect_words(child))
                    .filter(|word| !word.is_empty())
                    .collect::<Vec<_>>()
                    .join(" ")
            } else {
                String::new()
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



/// Function to unpack the children of a given `CCGNode`
pub fn unpack_children(maybe_nodes: Option<Vec<Box<CCGNode>>>) -> (CCGNode, CCGNode) {
    let nodes_vec = maybe_nodes.expect("Expected a vector of nodes, found None.");
    let first = nodes_vec.get(0).expect("Expected at least one node, found none.");
    let second = nodes_vec.get(1).expect("Expected at least two nodes, found only one.");
    ( (**first).clone(), (**second).clone() )
}
