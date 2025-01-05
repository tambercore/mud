use serde::{Deserialize, Deserializer};
use serde::de::{self, MapAccess, Visitor};
use std::fmt;
use crate::ccg::ccg_word::CCGWord;
use super::ccg_rule::CCGRule;
use super::ccg_type::CCGType;


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
        // Write the node type
        write!(f, "Type: {}", self.node_type)?;

        // If there's a word, write it
        if let Some(ref word) = self.word {
            write!(f, ", Word: {}", word)?;
        }

        // Write the rule
        write!(f, ", Rule: {}", self.rule)?;

        // If there are children, recursively format them
        if !self.children.clone().unwrap().is_empty() {
            write!(f, ", Children: [")?;
            for (i, child) in self.children.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?; // Add comma between children
                }
                write!(f, "{:?}", child)?; // Recursive call
            }
            write!(f, "]")?;
        }

        Ok(())
    }
}
