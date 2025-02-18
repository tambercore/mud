mod ccg;
mod brill;
mod lambda;
mod wordnet;
mod lingo;
mod monty;
mod type_theory;

use std::collections::HashMap;
use std::ptr::read;
use crate::brill::brill_tagger::tag_sentence;
use crate::brill::contextual_ruleset::parse_contextual_ruleset;
use crate::brill::init_tagger::initialize_tagger;
use crate::brill::lexical_ruleset::parse_lexical_ruleset;
use crate::ccg::sentence_parser::english_to_ccg;
use crate::monty::ccg_to_lc::*;
use crate::lambda::reducible::*;
use crate::lambda::types::{Expandable, LambdaEntity};
use crate::monty::typing_context::{reset_typing_context, TYPING_CONTEXT};

fn main() {

    let lexical_ruleset = parse_lexical_ruleset("data/rulefile_lexical.txt").unwrap();
    let contextual_ruleset = parse_contextual_ruleset("data/rulefile_contextual.txt").unwrap();
    let mut wc_mapping = initialize_tagger("data/lexicon.txt").unwrap();


    let sentence = "john likes cake";

    let vec_of_word_tag_tuples = tag_sentence(sentence, &lexical_ruleset, &contextual_ruleset, &mut wc_mapping);

    // println!("vec_word_tag_tuples: {:?}", vec_of_word_tag_tuples);

    // parse the ccg tree
    let mut ccg = english_to_ccg(sentence, vec_of_word_tag_tuples.clone());
    println!("Lambeq's CCG: \n{}", ccg);

    // Reset the typing context for each expression
    reset_typing_context();

    // CCG to lambda
    let lambda_expression = ccg_to_lambda(&mut ccg);
    println!("Result: \n{}", lambda_expression);
    // println!("context: \n{:?}", TYPING_CONTEXT.lock().unwrap());

    let reduction = (*lambda_expression).beta_reduce();
    println!("\n\nReduces to: \n{}", reduction);

    // let expanded_expression: Box<LambdaEntity> = (Box::from(reduction.expand()));
    // println!("expanded expression: {}", expanded_expression)
}

