use std::cmp::PartialEq;
use serde::Deserialize;
use std::fmt;
use crate::ccg::word::CCGWord;
use super::rule::CCGRule;
use super::category::CCGType;
use ascii_tree::{Tree::*, Tree, write_tree};
use uuid::Uuid;
use crate::lingo::quantifiers::{UNIVERSAL_QUANTIFIERS, EXISTENTIAL_QUANTIFIERS};

#[derive(Debug, Clone, Deserialize)]
pub struct CCGNode {

    #[serde(rename = "type")]
    pub node_type: CCGType,

    #[serde(rename = "text")]
    pub word: Option<CCGWord>,

    pub rule: CCGRule,
    pub children: Option<Vec<Box<CCGNode>>>, // Use Box to handle recursion

    // a unique identifier to differentiate nodes with the same text
    #[serde(skip)]
    pub id: Uuid
}


impl PartialEq for CCGNode {
    fn eq(&self, other: &Self) -> bool {
        // Compare node type
        if self.node_type != other.node_type {
            return false;
        }

        // Compare words (if they exist)
        if self.word != other.word {
            return false;
        }

        // Compare rules
        if self.rule != other.rule {
            return false;
        }

        if self.id != other.id {
            return false;
        }

        // Compare children
        match (&self.children, &other.children) {
            (Some(self_children), Some(other_children)) => {
                if self_children.len() != other_children.len() {
                    return false;
                }

                for (self_child, other_child) in self_children.iter().zip(other_children.iter()) {
                    if self_child != other_child {
                        return false;
                    }
                }
                true
            }
            (None, None) => true,
            _ => false,
        }
    }
}
impl CCGNode {
    /// Performs an in-order traversal of the CCGNode tree.
    /// Collects references to nodes in the provided vector in in-order sequence,
    /// but only pushes nodes that contain a `Some` word.
    pub fn inorder_traversal<'a>(&'a self, visit: &mut Vec<&'a CCGNode>) {
        // Recursive case
        if let Some(children) = &self.children {
            if children.len() >= 1 {
                children[0].inorder_traversal(visit);
            }
            if self.word.is_some() {
                visit.push(self);
            }
            if children.len() >= 2 {
                children[1].inorder_traversal(visit);
            }
        } else {
            if self.word.is_some() {
                visit.push(self);
            }
        }
    }

    /// Finds the parent of a given node within the tree.
    pub fn get_parent<'a>(&self, root: &'a CCGNode) -> Option<&'a CCGNode> {
        fn find_parent<'a>(
            current: &'a CCGNode,
            target: &CCGNode,
            parent: Option<&'a CCGNode>,
        ) -> Option<&'a CCGNode> {
            if current == target {
                return parent;
            }

            if let Some(children) = &current.children {
                for child in children {
                    if let Some(found) = find_parent(child, target, Some(current)) {
                        return Some(found);
                    }
                }
            }

            None
        }

        find_parent(root, self, None)
    }

    /// Finds a sibling of the given node within the tree.
    pub fn get_sibling<'a>(&self, root: &'a CCGNode) -> Option<&'a CCGNode> {
        if let Some(parent) = self.get_parent(root) {
            if let Some(children) = &parent.children {
                for child in children {
                    if **child != *self {
                        return Some(child);
                    }
                }
            }
        }
        None
    }

    /// Backtracks up from a node until there is a rhs child which does not contain a quantification node
    pub fn backtrack_until_rhs<'a>(&self, root: &'a CCGNode) -> Option<&'a CCGNode> {
        let mut current = self;
        while let Some(parent) = current.get_parent(root) {
            if let Some(children) = &parent.children {
                if let Some(rhs) = children.last() {
                    if **rhs != *current && !rhs.contains_quantification_node() {
                        return Some(rhs);
                    }
                }
            }
            current = parent;
        }
        None
    }

    pub fn backtrack_until_lhs<'a>(&self, root: &'a CCGNode) -> Option<&'a CCGNode> {
        let mut current = self;
        while let Some(parent) = current.get_parent(root) {
            if parent.node_type == CCGType::Sentence {
                if let Some(children) = &parent.children {
                    if let Some(lhs) = children.first() {
                        if !lhs.contains_quantification_node() {
                            return Some(lhs);
                        }
                    }
                }
            }
            current = parent;
        }
        None
    }

    /// Checks if a node or its descendants contain a quantification node.
    pub fn contains_quantification_node(&self) -> bool {
        if let Some(word) = &self.word {
            if UNIVERSAL_QUANTIFIERS.contains(&word.text) || EXISTENTIAL_QUANTIFIERS.contains(&word.text) {
                return true;
            }
        }

        if let Some(children) = &self.children {
            for child in children {
                if child.contains_quantification_node() {
                    return true;
                }
            }
        }
        false
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