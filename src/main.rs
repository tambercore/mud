mod ccg;
mod brill;
mod lambda;
mod wordnet;
mod lingo;
mod monty;
mod composer;
mod command_line;
mod server;
mod resolver;

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use warp::Filter;
use crate::brill::brill_tagger::{get_sentence_tags, tag_sentence};
use crate::brill::contextual_ruleset::parse_contextual_ruleset;
use crate::brill::init_tagger::{initialize_tagger, WordclassMap};
use crate::brill::lexical_ruleset::parse_lexical_ruleset;
use crate::brill::utils::{create_tag_mapping, TAG_MAPPING};
use crate::ccg::sentence_parser::english_to_ccg;
use crate::monty::ccg_to_lc::*;
use crate::lambda::reducible::*;
use crate::lambda::types::{Expandable, LambdaEntity};
use crate::monty::typing_context::{reset_typing_context, TYPING_CONTEXT};
use crate::composer::postulate::{initialise_agda_file, AgdaFile};
use crate::composer::agdaify::*;
use crate::composer::lambda_to_types::compose;
use crate::command_line::get_arguments::{Config};
use crate::composer::knowledge_base::{compose_kb, KnowledgeBase};
use crate::composer::structures::AgdaType;
// use crate::server::server::create_endpoint;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use crate::brill::contextual_rulespec::ContextualRulespec;
use crate::brill::lex_rulespec_id::LexicalRulespec;
use crate::brill::wordclass::Wordclass;
use crate::composer::conclusions::compose_conclusions;
use crate::composer::langtree::{lambda_to_semantic, SemanticTree};
use crate::lambda::etalike::Eliminator;
use crate::resolver::fill_holes::fill_holes;
use crate::server::server::create_endpoint;
// use crate::resolver::fill_holes::fill_holes;

// Assuming these types exist in your code:
struct LexicalRuleset { /* ... */ }
struct ContextualRuleset { /* ... */ }

static LEXICAL_RULESET: Lazy<Vec<LexicalRulespec>> = Lazy::new(|| {
    parse_lexical_ruleset("data/rulefile_lexical.txt").unwrap()
});
static CONTEXTUAL_RULESET: Lazy<HashMap<Wordclass, Vec<ContextualRulespec>>> = Lazy::new(|| {
    parse_contextual_ruleset("data/rulefile_contextual.txt").unwrap()
});
static WC_MAPPING: Lazy<Mutex<WordclassMap>> = Lazy::new(|| {
    Mutex::new(initialize_tagger("data/lexicon.txt").unwrap())
});

fn sentence_to_agda(sentence: String, f: &mut AgdaFile) -> ((String, AgdaType), String) {

    /* Access the global references for the brill tagger! */
    let lexical_ruleset = &*LEXICAL_RULESET;
    let contextual_ruleset = &*CONTEXTUAL_RULESET;
    let mut wc_mapping = WC_MAPPING.lock().unwrap();

    let possible_tags = get_sentence_tags(&sentence, &mut wc_mapping);
    let vec_of_word_tag_tuples = tag_sentence(&sentence, lexical_ruleset, contextual_ruleset, &mut wc_mapping);
    create_tag_mapping(possible_tags, vec_of_word_tag_tuples.clone());
    println!("tag mapping: {:?}", TAG_MAPPING.get().unwrap());

    let (mut ccg, json_tree) = english_to_ccg(&sentence, vec_of_word_tag_tuples.clone());
    println!("Lambeq's CCG: \n{}", ccg);

    let lambda_expression = ccg_to_lambda(&mut ccg);
    println!("Result: \n{}", lambda_expression);

    let reduction = (*lambda_expression).beta_reduce();
    println!("\n\nReduces to: \n{}", reduction);

    let eta_reduction = (reduction).eliminate_leftovers();
    println!("\n\nEta Reduces to: \n{}", eta_reduction);

    let expanded_expression: Box<LambdaEntity> = Box::from(eta_reduction.expand());
    println!("\n\nExpands to: {}", expanded_expression);

    let semantic_tree = lambda_to_semantic(Box::from(expanded_expression.clone())).expect("Failed to parse semantic tree.");

    let encoded_sentence = compose(Box::from(semantic_tree), f, vec![]);
    (encoded_sentence, json_tree)
}

fn english_to_agda(knowledge: Vec<String>, conclusions: Vec<String>) -> (AgdaFile, Vec<String>, Vec<String>) {

    /* Initialise the Agda File (get it ready) */
    let mut f = initialise_agda_file();

    /* Initialise an empty vector to hold each CCG in JSON form. */
    let mut premise_trees = Vec::new();
    let mut conclusion_trees = Vec::new();

    /* Handle Assumptions */
    let mut encoded_knowledge: KnowledgeBase = vec![];
    for sentence in knowledge {
        let (encoded_sentence, ccg_json) = sentence_to_agda(sentence, &mut f);
        encoded_knowledge.push(encoded_sentence);
        premise_trees.push(ccg_json);
    }
    compose_kb(encoded_knowledge, &mut f);

    /* Handle Conclusions */
    let mut encoded_conclusions: Vec<(String, AgdaType)> = vec![];
    for conclusion in conclusions {
        let (encoded_conclusion, ccg_json) = sentence_to_agda(conclusion, &mut f);
        encoded_conclusions.push(encoded_conclusion);
        conclusion_trees.push(ccg_json);
    }
    compose_conclusions(encoded_conclusions, &mut f);

    (f, premise_trees, conclusion_trees)
}




#[tokio::main]
async fn main() {
    let config = Config::from_args("socrates is a man & every man is mortal -> socrates is mortal");
    let knowledge = config.knowledge;
    let conclusions = config.conclusions;

    /* If config.server, create an endpoint and wait for client requests. */
    if config.server {
        create_endpoint(config.output_file).await;
    }

    /* Run locally and save agda as a file. */
    else {
        let (mut agda_file, premises, conclusions) = english_to_agda(knowledge, conclusions);
        agda_file.write_to_file(config.output_file.clone());
        fill_holes(config.output_file.clone());
    }
}
