use std::error::Error;
use std::io::ErrorKind;
use crate::lambda::types::LambdaEntity;
use ascii_tree::{Tree::*, Tree, write_tree};
use std::fmt;

pub enum SemanticTree {
    Predicate(String, Vec<Box<SemanticTree>>),
    Terminal(String),
    Conjunction(Box<SemanticTree>, Box<SemanticTree>),
}

impl fmt::Display for SemanticTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn to_ascii_tree(node: &SemanticTree) -> Tree {
            match node {
                SemanticTree::Predicate(pred, args) => {
                    // Title text for the current node
                    let title = format!("Predicate: {}", pred);

                    // Build child nodes recursively from arguments
                    let children = args
                        .iter()
                        .map(|child| to_ascii_tree(child))
                        .collect::<Vec<_>>();
                    Node(title, children)
                }

                SemanticTree::Terminal(t) => {
                    // Terminal node has no children
                    Node(format!("Terminal: {}", t), vec![])
                }

                SemanticTree::Conjunction(lhs, rhs) => {
                    // Conjunction node with two children
                    let title = "Conjunction".to_string();
                    let left_child = to_ascii_tree(lhs);
                    let right_child = to_ascii_tree(rhs);
                    Node(title, vec![left_child, right_child])
                }
            }
        }

        // Build the ASCII tree structure
        let ascii_tree = to_ascii_tree(self);

        // Write the tree to a String, and then write that String to the Formatter
        let mut output = String::new();
        write_tree(&mut output, &ascii_tree)
            .map_err(|_| fmt::Error)?; // Convert any I/O-ish errors to fmt::Error

        write!(f, "{}", output)
    }
}

pub fn lambda_to_semantic(node: Box<LambdaEntity>) -> Result<SemanticTree, String> {
    match *node {
        LambdaEntity::Var(v) => Ok(SemanticTree::Terminal(v.name.clone())),

        LambdaEntity::Pred(p) => {
            let mut converted_args: Vec<Box<SemanticTree>> = vec![];
            for arg in p.args {
                let converted_arg = lambda_to_semantic(arg)?;
                converted_args.push(Box::from(converted_arg));
            }
            Ok(SemanticTree::Predicate(p.iden.clone(), converted_args))
        }

        LambdaEntity::Conj(conj) => {
            let lhs = lambda_to_semantic(conj.lhs)?;
            let rhs = lambda_to_semantic(conj.rhs)?;
            Ok(SemanticTree::Conjunction(Box::new(lhs), Box::new(rhs)))
        }

        LambdaEntity::App(_) => Err("Can't convert Application".to_string()),

        LambdaEntity::Abs(_) => Err("Can't convert Abstraction".to_string()),
    }
}