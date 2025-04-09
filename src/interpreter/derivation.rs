use std::collections::HashMap;
use crate::brill::brill_tagger::tag_sentence;
use crate::brill::wordclass::Wordclass;
use crate::{CONTEXTUAL_RULESET, LEXICAL_RULESET, WC_MAPPING};
use crate::ast::top_decl::TDeclaration;

#[derive(Debug, Clone)]
pub struct Assumption {
    pub contents: String,
    pub expr: TDeclaration,
    pub Id: String,
}
#[derive(Debug, Clone)]
pub struct DerivationNode {
    pub derivation: Derivation,
    pub children: Vec<DerivationNode>,
}

#[derive(Debug, Clone)]
pub struct Derivation {
    pub contents: String,
    pub expr: TDeclaration,
}


pub fn print_assumptions(assumptions: &Vec<Assumption>) {
    for assumption in assumptions {
        println!("{} : {}", assumption.Id, assumption.contents);
    }
}

pub fn print_derivation_node(node: &DerivationNode) {
    println!("TEMP : {}.", node.derivation.contents, node.children);
    for child in &node.children {
        print_derivation_node(child);
    }
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

