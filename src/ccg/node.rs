use std::cmp::PartialEq;
use serde::Deserialize;
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

    #[serde(skip)]
    pub is_quantification_node: bool
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
        println!("Searching parent of node: {}", self);
        fn find_parent<'a>(
            current: &'a CCGNode,
            target: &CCGNode,
            parent: Option<&'a CCGNode>,
        ) -> Option<&'a CCGNode> {
            if current == target{
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


    // Recursive function to initialize flags
    pub fn initialize_flags(&mut self, every_quantifiers: Vec<String>) {
        // Set the flag to false by default
        self.is_quantification_node = false;

        if let Some(children) = self.clone().children {
            for c in children {
                if let Some(grandchildren) = c.children {
                    for g in grandchildren {
                        if let Some(ccg_word) = g.word {
                            if every_quantifiers.contains(&ccg_word.text.to_lowercase()) {
                                self.is_quantification_node = true;
                                //return;
                            }
                        }
                    }
                }
            }
        }

        if let Some(children) = &mut self.children {
                for child in children.iter_mut() {
                    child.initialize_flags(every_quantifiers.clone());
                    if child.is_quantification_node {
                        if let Some(grandchildren) = &mut child.children {
                            for grandchild in grandchildren.iter_mut() {
                                grandchild.initialize_flags(every_quantifiers.clone());
                                if grandchild.is_quantification_node {
                                    grandchild.is_quantification_node = false;
                                    child.is_quantification_node = false;
                                    self.is_quantification_node = true;
                                }
                            }
                        }
                    }
            }
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