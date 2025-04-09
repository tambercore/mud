use std::cmp::PartialEq;
use std::collections::HashMap;
use serde::de::Unexpected::Str;
use crate::brill::brill_tagger::tag_sentence;
use crate::brill::wordclass::Wordclass;
use crate::{CONTEXTUAL_RULESET, LEXICAL_RULESET, WC_MAPPING};
use crate::ast::program::Program;
use crate::ast::top_decl::TDeclaration;

#[derive(Debug, Clone)]
pub struct Assumption {
    pub contents: String,
    pub expr: TDeclaration,
}

#[derive(Debug, Clone)]
pub struct DerivationInformation {
    pub assumptions: Vec<Assumption>,
    pub derivations: DerivationNode,
    pub program: Program
}

#[derive(Debug, Clone)]
pub struct DerivationNode {
    pub derivation: Derivation,
    pub parent: Option<Box<DerivationNode>>,
    pub children: Vec<DerivationNode>,
}

#[derive(Debug, Clone)]
pub struct Derivation {
    pub contents: String,
    pub expr: TDeclaration,
}


pub fn print_assumptions(assumptions: &Vec<Assumption>) {

    for (idx, assumption) in assumptions.iter().enumerate() {
        println!("A{} : {}", idx, assumption.contents);
    }
}

pub fn print_derivation_node(node: &DerivationNode) {
    _print_derivation_node(node, String::from("1"), 0);
}

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







impl Derivation {
    pub fn get_tag(&self, word: &str) -> Option<Wordclass> {
        /* Access the global references for the brill tagger! */
        let lexical_ruleset = &*LEXICAL_RULESET;
        let contextual_ruleset = &*CONTEXTUAL_RULESET;
        let mut wc_mapping = WC_MAPPING.lock().unwrap();

        let vec_of_word_tag_tuples = tag_sentence(&self.contents, lexical_ruleset, contextual_ruleset, &mut wc_mapping);

        // Find the tuple where the word matches, and return the associated tag
        vec_of_word_tag_tuples.iter()
            .find(|(w, _)| w == word)
            .map(|(_, tag)| *tag)
    }
}

