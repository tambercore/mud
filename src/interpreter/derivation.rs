use std::collections::HashMap;
use crate::brill::brill_tagger::tag_sentence;
use crate::brill::wordclass::Wordclass;
use crate::{CONTEXTUAL_RULESET, LEXICAL_RULESET, WC_MAPPING};
use crate::ast::top_decl::TDeclaration;

pub struct Assumption {
    pub contents: String,
    pub expr: TDeclaration,
    pub Id: String,
}

pub struct Derivation {
    pub contents: String,
    pub expr: TDeclaration,
    pub Id: String,
}

pub fn print_derivations(lines: &Derivations) {
    println!("Derivations: ");
    for line in &lines.contents {
        println!("{} : {}", line.Id, line.contents);
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

pub struct Derivations {
    pub(crate) contents: Vec<Derivation>,
    pub assumptions: Vec<Assumption>
}

impl Derivations {
    pub fn find_id_by_contents(&self, contents: &str) -> Option<&str> {
        self.contents.iter()
            .find(|d| d.contents == contents)
            .map(|d| d.Id.as_str())
            .or_else(|| {
                self.assumptions.iter()
                    .find(|a| a.contents == contents)
                    .map(|a| a.Id.as_str())
            })
    }

    pub fn find_id_by_expr(&self, expr: TDeclaration) -> Option<&str> {
        self.contents.iter()
            .find(|d| d.expr == expr)
            .map(|d| d.Id.as_str())
            .or_else(|| {
                self.assumptions.iter()
                    .find(|a| a.expr == expr)
                    .map(|a| a.Id.as_str())
            })
    }
}
