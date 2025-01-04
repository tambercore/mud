use crate::ccg::ccg_types::{get_terminal_nodes, CCGCategory, CCGNode, CCGRule};
use crate::ccg::language_parser::english_to_ccg;
use crate::brill::brill_tagger::tag_sentence;
use crate::brill::contextual_ruleset::parse_contextual_ruleset;
use crate::brill::init_tagger::initialize_tagger;
use crate::brill::lexical_ruleset::parse_lexical_ruleset;
use crate::brill::wordclass::Wordclass;
use crate::ccg::ccg_types::CCGOperator::{Backward, Forward};
use crate::ccg::ccg_types::CCGRule::FA;
use crate::lambda::reduce::reduce;
use crate::lambda::types::LambdaEntity;
use crate::lambda::types::LambdaEntity::Variable;
use crate::montague::montague_grammar::map_word_to_expression;

fn ccg_to_fol(ccg: CCGNode) {
    print_nodes_with_text(&ccg);
}

/// Traverses the CCGNode and prints out nodes that have a "text" value.
fn print_nodes_with_text(node: &CCGNode) {
    // If the node has a text value, print it.
    if let Some(ref text) = node.text {
        println!("Terminal node: {}", node);
    }

    // Recursively traverse the child nodes if they exist.
    if let Some(ref children) = node.children {
        for child in children {
            print_nodes_with_text(child); // Recursively print for each child.
        }
    }
}


#[test]
fn run() {
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

    println!("ccg: {}", ccg);

    //Sreturn;

    // map words to their montague grammar representation
    // todo : implement rest of ccgtypes
    let montague_representation = map_words_to_montague(vec_of_word_tag_tuples.clone(), &ccg);

    println!("{:?}", montague_representation);

    // reduce the montague grammar into a single fol expression
    let fol = reduce_montague(&montague_representation, &ccg);



    println!("REDUCED FOL: {:?}", fol);
}

/// Reduce individual FOL words into the complete FOL sentence representation using rules from the CCG tree.
fn reduce_montague(terminals_to_fol: &Vec<(String, LambdaEntity)>, ccg_tree: &CCGNode) -> Result<LambdaEntity, String> {
    // TODO
    Err("not implemented".parse().unwrap())
}

/// Map word tag pairs to their corresponding montague grammar representation using the CCG tree.
fn map_words_to_montague(words_to_tags : Vec<(String, Wordclass)>, ccg : &CCGNode) -> Vec<(String, LambdaEntity)> {
    let terminal_nodes = get_terminal_nodes(&ccg);
    let mut word_to_lambda: Vec<(String, LambdaEntity)> = Vec::new();

    for node in terminal_nodes {
        // find the entry of words_to_tags where word == node.text
        if let Some(word) = &node.text {
            // Find the entry in `words_to_tags` where the word matches `node.text`.
            if let Some((word, wordclass)) = words_to_tags.iter().find(|(w, _)| w == word) {
                let expr = map_word_to_expression(String::from(word), wordclass, node).expect("could not map");
                word_to_lambda.push((String::from(word), expr))
            }
        }
    }
    word_to_lambda

}