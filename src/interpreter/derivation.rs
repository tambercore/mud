use std::cmp::PartialEq;
use std::collections::HashMap;
use serde::de::Unexpected::Str;
use crate::brill::brill_tagger::tag_sentence;
use crate::brill::wordclass::Wordclass;
use crate::{CONTEXTUAL_RULESET, LEXICAL_RULESET, WC_MAPPING};
use crate::ast::program::Program;
use crate::ast::top_decl::TDeclaration;



/// Represents an assumption in a derivation system.
///
/// Contains the content (description) of the assumption and its associated expression.
#[derive(Debug, Clone)]
pub struct Assumption {
    pub contents: String,
    pub expr: TDeclaration,
}



/// Holds information about a derivation, including assumptions and the program being used.
#[derive(Debug, Clone)]
pub struct DerivationInformation {
    pub assumptions: Vec<Assumption>,
    pub derivations: DerivationNode,
    pub program: Program
}



/// Represents a node in the derivation tree.
///
/// A `DerivationNode` contains a derivation, its parent node (if any), and any child nodes.
/// It forms part of the hierarchical structure that represents a proof or logical derivation.
#[derive(Debug, Clone)]
pub struct DerivationNode {
    pub derivation: Derivation,
    pub parent: Option<Box<DerivationNode>>,
    pub children: Vec<DerivationNode>,
}



/// Represents a single derivation within a derivation tree.
///
/// A `Derivation` contains the contents (description) of the derivation and the associated Agda expression.
#[derive(Debug, Clone)]
pub struct Derivation {
    pub contents: String,
    pub expr: TDeclaration,
}



/// Prints out all assumptions in a human-readable format.
pub fn print_assumptions(assumptions: &Vec<Assumption>) {
    for (idx, assumption) in assumptions.iter().enumerate() {
        println!("A{} : {}", idx, assumption.contents);
    }
}



/// Prints the derivation tree starting from the root node.
pub fn print_derivation_node(node: &DerivationNode) {
    _print_derivation_node(node, String::from("1"), 0);
}



/// Recursively prints the derivation tree, including child nodes.
pub fn _print_derivation_node(node: &DerivationNode, ind: String, pos: i32) {
    let mut judgment_id_least_sig: String;
    if pos == 0 {
        judgment_id_least_sig = String::from(format!("{}", ind));
    } else {
        judgment_id_least_sig = String::from(format!("{}.{}", ind, pos));
    }

    println!("{} : {}.", judgment_id_least_sig.clone(), node.derivation.contents);
    for (pos, child) in node.children.iter().enumerate() {
        _print_derivation_node(child, judgment_id_least_sig.clone(), (pos as i32) + 1);
    }
}



/// Gets the derivation ID for a target expression from a derivation tree.
pub fn get_derivation_id(node: &DerivationNode, target: String, assumptions: &Vec<Assumption>) -> String {

    for (idx, assumption) in assumptions.iter().enumerate() {
        if assumption.contents == target {
            return format!("A{}", idx);
        }
    }

    fn find_root(node: &DerivationNode) -> &DerivationNode {
        // Start from the current node and keep going up the tree
        let mut current_node = node;
        while let Some(ref parent) = current_node.parent {
            current_node = parent;
        }
        current_node
    }

    // Find the root node first
    let root = find_root(node);

    _get_derivation_id(root, String::from("1"), 0, target)
}



/// Recursively finds and returns the derivation ID for a target expression from the derivation tree.
pub fn _get_derivation_id(node: &DerivationNode, ind: String, pos: i32, target: String) -> String {

    if target == node.derivation.contents {
        return node.clone().derivation.contents;
    }

    let mut judgment_id_least_sig: String;
    if pos == 0 {
        judgment_id_least_sig = String::from(format!("{}", ind));
    } else {
        judgment_id_least_sig = String::from(format!("{}.{}", ind, pos));
    }

    println!("{} : {}.", judgment_id_least_sig.clone(), node.derivation.contents);
    for (pos, child) in node.children.iter().enumerate() {
        let pot_id = _get_derivation_id(child, judgment_id_least_sig.clone(), (pos as i32) + 1, target.clone());
        if pot_id != "Unknown" {
            return pot_id;
        }
    }
    return String::from("Unknown");
}
