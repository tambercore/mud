use std::collections::HashMap;
use crate::brill::brill_tagger::tag_sentence;
use crate::brill::wordclass::Wordclass;
use crate::{CONTEXTUAL_RULESET, LEXICAL_RULESET, WC_MAPPING};
use crate::ast::top_decl::TDeclaration;

#[derive(Clone)]
pub struct Assumption {
    pub contents: String,
    pub expr: TDeclaration,
    pub Id: String,
}
#[derive(Clone)]
pub struct DerivationNode {
    pub derivation: Derivation,
    pub children: Vec<DerivationNode>,
}

#[derive(Clone)]
pub struct Derivations {
    pub(crate) contents: DerivationNode,
    pub assumptions: Vec<Assumption>
}
#[derive(Clone)]
pub struct Derivation {
    pub contents: String,
    pub expr: TDeclaration,
    pub Id: String,
}

pub fn print_derivations(derivations: &Derivations) {
    print_assumptions(&derivations.assumptions);
    print_derivation_node(&derivations.contents, 0);
}

fn print_assumptions(assumptions: &Vec<Assumption>) {
    for assumption in assumptions {
        println!("{} : {}", assumption.Id, assumption.contents);
    }
}

fn print_derivation_node(node: &DerivationNode, indent: usize) {
    let prefix = "  ".repeat(indent);
    println!("{}{} : {}", prefix, node.derivation.Id, node.derivation.contents);
    for child in &node.children {
        print_derivation_node(child, indent + 1);
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

impl Derivations {
    pub fn find_id_by_contents(&self, contents: String) -> Option<String> {
        Self::find_in_tree_by_contents(self.contents.clone(), contents.clone())
            .or_else(|| {
                self.assumptions.iter()
                    .find(|a| a.contents == contents)
                    .map(|a| a.Id.clone())
            })
    }

    fn find_in_tree_by_contents(node: DerivationNode, contents: String) -> Option<String> {
        if node.derivation.contents == contents.clone() {
            return Some(node.derivation.Id);
        }

        for child in node.children {
            if let Some(id) = Self::find_in_tree_by_contents(child, contents.clone()) {
                return Some(id);
            }
        }

        None
    }

    pub fn find_id_by_expr(&self, expr: TDeclaration) -> Option<String> {
        Self::find_in_tree_by_expr(self.clone().contents, expr.clone())
            .or_else(|| {
                self.assumptions.iter()
                    .find(|a| a.expr == expr)
                    .map(|a| a.Id.clone())
            })
    }

    fn find_in_tree_by_expr(node: DerivationNode, expr: TDeclaration) -> Option<String> {
        if node.derivation.expr == expr {
            return Some(node.derivation.Id);
        }

        for child in node.children {
            if let Some(id) = Self::find_in_tree_by_expr(child, expr.clone()) {
                return Some(id);
            }
        }

        None
    }
}
