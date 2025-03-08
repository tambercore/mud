mod ccg;
mod brill;
mod lambda;
mod wordnet;
mod lingo;
mod monty;
mod composer;
mod command_line;
mod server;

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use warp::Filter;
use crate::brill::brill_tagger::{get_sentence_tags, tag_sentence};
use crate::brill::contextual_ruleset::parse_contextual_ruleset;
use crate::brill::init_tagger::initialize_tagger;
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


fn english_to_agda(knowledge: Vec<String>, conclusions: Vec<String>) -> AgdaFile {

    /* Initializing the Brill Tagger with its lexical and contextual rulesets. */
    let lexical_ruleset = parse_lexical_ruleset("data/rulefile_lexical.txt").unwrap();
    let contextual_ruleset = parse_contextual_ruleset("data/rulefile_contextual.txt").unwrap();
    let mut wc_mapping = initialize_tagger("data/lexicon.txt").unwrap();

    /* Initialise the Agda File (get it ready) */
    let mut f = initialise_agda_file();

    /* This is per sentence! */
    let mut encoded_knowledge: KnowledgeBase = vec![];
    for sentence in knowledge {
        let possible_tags = get_sentence_tags(&sentence, &mut wc_mapping);
        let vec_of_word_tag_tuples = tag_sentence(&sentence, &lexical_ruleset, &contextual_ruleset, &mut wc_mapping);
        create_tag_mapping(possible_tags, vec_of_word_tag_tuples.clone());
        println!("tag mapping: {:?}", TAG_MAPPING.get().unwrap());

        let mut ccg = english_to_ccg(&sentence, vec_of_word_tag_tuples.clone());
        println!("Lambeq's CCG: \n{}", ccg);

        let lambda_expression = ccg_to_lambda(&mut ccg);
        println!("Result: \n{}", lambda_expression);

        let reduction = (*lambda_expression).beta_reduce();
        println!("\n\nReduces to: \n{}", reduction);

        let expanded_expression: Box<LambdaEntity> = (Box::from(reduction.expand()));
        println!("\n\nExpands to: {}", expanded_expression);

        let encoded_sentence = compose(expanded_expression, &mut f, vec![]);
        encoded_knowledge.push(encoded_sentence);
    }
    compose_kb(encoded_knowledge, &mut f);

    f
}




#[tokio::main]
async fn main() {
    let config = Config::from_args("john is happy");
    let sentence = config.sentence;

    /* If config.server, create an endpoint and wait for client requests. */
    if config.server {
        // create_endpoint().await;

    }

    /* Run locally and save agda as a file. */
    else {
        let knowledge = vec![String::from("a john is a man"), String::from("every man is an animal")];
        let conclusions = vec![String::from("a john is an animal")];

        let mut agda_file = english_to_agda(knowledge, conclusions);
        agda_file.write_to_file(config.output_file);
    }
}
