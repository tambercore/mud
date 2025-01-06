mod ccg;
mod brill;
mod lambda;
mod wordnet;
mod lingo;
mod monty;

use std::io::{self, Write};
use crate::brill::brill_tagger::tag_sentence;
use crate::brill::contextual_ruleset::parse_contextual_ruleset;
use crate::brill::init_tagger::initialize_tagger;
use crate::brill::lexical_ruleset::parse_lexical_ruleset;
use crate::ccg::sentence_parser::english_to_ccg;
use crate::ccg::type_parser::*;
use crate::lingo::past_participle::get_past_participle;
use crate::monty::lambda_generation::ccg_to_lambda;
use crate::lambda::reduce::reduce;

fn main() {
    let lexical_ruleset = parse_lexical_ruleset("data/rulefile_lexical.txt").unwrap();
    let contextual_ruleset = parse_contextual_ruleset("data/rulefile_contextual.txt").unwrap();
    let mut wc_mapping = initialize_tagger("data/lexicon.txt").unwrap();

    // TODO: contractions break the tagger (don't does not get a tag etc)

    let sentence = "All ravens are black";

    // retrieve words and their corresponding pos tags
    let vec_of_word_tag_tuples = tag_sentence(sentence, &lexical_ruleset, &contextual_ruleset, &mut wc_mapping);

    println!("vec_word_tag_tuples: {:?}", vec_of_word_tag_tuples);

    // parse the ccg tree
    let ccg = english_to_ccg(sentence, vec_of_word_tag_tuples.clone());
    println!("ccg: \n{}", ccg);

    // CCG to lambda
    let lambda_expression = ccg_to_lambda(ccg);
    println!("lambda: \n{}", lambda_expression);

    let reduction = reduce(&*lambda_expression);
    println!("reduced expression: \n{}", reduction);
}