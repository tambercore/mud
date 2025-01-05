use serde::{Deserialize, Deserializer};
use serde::de::{self, MapAccess, Visitor};
use std::fmt;
use crate::ccg::word::CCGWord;
use super::rule::CCGRule;
use super::r#type::CCGType;
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