use serde::Deserialize;
use std::fmt;
use crate::ccg::word::CCGWord;
use super::rule::CCGRule;
use super::category::CCGType;
use ascii_tree::{Tree::*, Tree, write_tree};
use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug, Clone)]
pub struct CCGNode {

    pub node_type: CCGType,

    pub word: Option<CCGWord>,

    pub rule: CCGRule,

    /// Strong references to children
    pub children: RefCell<Vec<Rc<RefCell<CCGNode>>>>,

    /// Weak reference to parent (to avoid cycles)
    pub parent: RefCell<Option<Weak<RefCell<CCGNode>>>>,
}

impl CCGNode {
    /// Creates a new CCGNode wrapped in `Rc<RefCell<CCGNode>>`.
    pub fn new(
        node_type: CCGType,
        word: Option<CCGWord>,
        rule: CCGRule
    ) -> Rc<RefCell<CCGNode>> {
        Rc::new(RefCell::new(CCGNode {
            node_type,
            word,
            rule,
            children: RefCell::new(vec![]),
            parent: RefCell::new(None),
        }))
    }

    /// In-order traversal, collecting `Rc<RefCell<CCGNode>>` in `visit`.
    /// Assumes at most 2 children.
    pub fn inorder_traversal(node_rc: &Rc<RefCell<CCGNode>>, visit: &mut Vec<Rc<RefCell<CCGNode>>>) {
        let binding = node_rc.borrow();
        let children = binding.children.borrow();

        // If there's a left child, recurse
        if children.len() >= 1 {
            Self::inorder_traversal(&children[0], visit);
        }

        // Add the current node
        visit.push(Rc::clone(node_rc));

        // If there's a right child, recurse
        if children.len() >= 2 {
            Self::inorder_traversal(&children[1], visit);
        }
    }

    /// Performs an in-order traversal of the (binary) tree rooted at `root`
    /// **but** returns only the leaf nodes (i.e. zero children).
    /// The returned leaf nodes are each `Box<CCGNode>` (a brand-new boxed clone of each leaf).
    pub fn inorder_leaves(root: &CCGNode) -> Vec<Box<CCGNode>> {
        let mut leaves = Vec::new();

        // Helper function for recursion
        fn helper(node: &CCGNode, out: &mut Vec<Box<CCGNode>>) {
            let children = node.children.borrow();
            match children.len() {
                0 => {
                    // Leaf node => collect it
                    out.push(Box::new(node.clone()));
                }
                1 => {
                    // Single child => "in-order" just goes down that child
                    helper(&children[0].borrow(), out);
                }
                2 => {
                    // Standard in-order on a binary node:
                    // 1) Left child
                    helper(&children[0].borrow(), out);
                    // (Skip the parent itself, since we only want leaves)
                    // 2) Right child
                    helper(&children[1].borrow(), out);
                }
                _ => {
                    // If you allow more than 2 children, define how "in-order" works.
                    for child_rc in children.iter() {
                        helper(&child_rc.borrow(), out);
                    }
                }
            }
        }

        helper(root, &mut leaves);
        leaves
    }
}



impl fmt::Display for CCGNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn collect_words(node: &CCGNode) -> String {
            if let Some(word) = &node.word {
                // Terminal node with a word
                word.text.clone()
            } else {
                // Recursively gather words from children
                let children = node.children.borrow();
                children
                    .iter()
                    .map(|child_rc| collect_words(&child_rc.borrow()))
                    .filter(|w| !w.is_empty())
                    .collect::<Vec<_>>()
                    .join(" ")
            }
        }

        fn to_ascii_tree(node: &CCGNode) -> Tree {
            let aggregated_word = collect_words(node);
            let title = format!(
                "'{}', {}, [{}]",
                aggregated_word,
                node.node_type,
                node.rule
            );

            // Build ASCII children recursively
            let children = node
                .children
                .borrow()
                .iter()
                .map(|child_rc| to_ascii_tree(&child_rc.borrow()))
                .collect();

            Node(title, children)
        }

        // Build the ASCII tree from `self`
        let ascii_tree = to_ascii_tree(self);

        // Write it out
        let mut output = String::new();
        write_tree(&mut output, &ascii_tree).map_err(|_| fmt::Error)?;
        write!(f, "{}", output)
    }
}