mod ccg;
mod brill;
mod lambda;
mod wordnet;
mod lingo;
mod monty;
mod composer;
mod command_line;

use std::collections::HashMap;
use std::ptr::read;
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
use crate::composer::postulate::initialise_agda_file;
use crate::composer::agdaify::*;
use crate::composer::lambda_to_types::compose;
use crate::command_line::get_arguments;
use crate::command_line::get_arguments::{handle_arguments, Config};

fn main() -> Result<(), i32> {

    /* For now, sentences may be hard-coded in the program. Using `-i` will overwrite this. */
    let sentence ="john is happy";
    let config = Config::from_args(sentence);

    /* Handle CLI arguments gracefully.
       If an argument requires the file to terminate, terminate in `main`. */
    if let Err(_) = handle_arguments(&config) {
        return Ok(())
    }

    let sentence = config.sentence;

    let mut f = initialise_agda_file();

    let lexical_ruleset = parse_lexical_ruleset("data/rulefile_lexical.txt").unwrap();
    let contextual_ruleset = parse_contextual_ruleset("data/rulefile_contextual.txt").unwrap();
    let mut wc_mapping = initialize_tagger("data/lexicon.txt").unwrap();

    let possible_tags = get_sentence_tags(&sentence, &mut wc_mapping);
    let vec_of_word_tag_tuples = tag_sentence(&sentence, &lexical_ruleset, &contextual_ruleset, &mut wc_mapping);
    create_tag_mapping(possible_tags, vec_of_word_tag_tuples.clone());
    println!("tag mapping: {:?}", TAG_MAPPING.get().unwrap());


    // Parse into the ccg tree
    let mut ccg = english_to_ccg(&sentence, vec_of_word_tag_tuples.clone());
    println!("Lambeq's CCG: \n{}", ccg);

    // Reset the typing context for each expression
    reset_typing_context();

    // CCG to lambda
    let lambda_expression = ccg_to_lambda(&mut ccg);
    println!("Result: \n{}", lambda_expression);

    // Reduce, then expand.
    let reduction = (*lambda_expression).beta_reduce();
    println!("\n\nReduces to: \n{}", reduction);

    let expanded_expression: Box<LambdaEntity> = (Box::from(reduction.expand()));
    println!("\n\nExpands to: {}", expanded_expression);

    let _ = compose(expanded_expression, &mut f, vec![]);

    //println!("{}", &f.clone().agdaify());

    f.write_to_file("output_file");

    Ok(())
}

/*
a green cheese is a weird myth

to have green cheese is to have a weird myth

Pi (e : GreenCheese) -> isWeirdMyth e

a cheese is a food

*/