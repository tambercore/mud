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

    return;

    // map words to their montague grammar representation
    let montague_representation = map_words_to_montague(vec_of_word_tag_tuples.clone(), &ccg);

    println!("{:?}", montague_representation);

    // reduce the montague grammar into a single fol expression
    let fol = reduce_montague(&montague_representation, &ccg);



    println!("REDUCED FOL: {:?}", fol);
}

/// Reduce individual FOL words into the complete FOL sentence representation using rules from the CCG tree.
fn reduce_montague(terminals_to_fol: &Vec<(String, LambdaEntity)>, ccg_tree: &CCGNode) -> Result<LambdaEntity, String> {
    fn apply_montague_recursively(
        node: &CCGNode,
        terminals_to_fol: &Vec<(String, LambdaEntity)>,
    ) -> Result<LambdaEntity, String> {
        // Base case: If the node is a terminal, return its Montague term
        if node.children == None {
            if let Some(ref text) = node.text {
                // Find the corresponding Montague term for the terminal
                if let Some((_, lambda_entity)) = terminals_to_fol.iter().find(|(word, _)| word == text) {
                    //println!("BASE CASE, TERMINAL: {:?}", lambda_entity.clone());
                    return Ok(lambda_entity.clone());
                }
            }
            panic!("Terminal node without a corresponding Montague term.");
        }

        // Recursive case: Process child nodes
        let children = node
            .children
            .as_ref()
            .expect("Non-terminal node without children.");

        // Combine child terms based on the CCG rule
        match node.clone().rule {
            Some(FA) => {
                println!("doing forward application on {:?}: children {:?} and {:?}\n",  node.category, children[0].category, children[1].category);

                // Forward Application: A \ B

                let target_category = node.clone().category;

                match (children[0].clone().category, children[1].clone().category) {

                    (CCGCategory::Composed {left, right, operator }, c) => {
                        println!("matched FA case 1. operator = {}. {:?} = {:?} and {:?} = {:?}", operator, left, target_category, right, c);

                        if operator == Forward {
                            if *right == c && *left == target_category {
                                let function_term = apply_montague_recursively(&children[0], terminals_to_fol);
                                let argument_term = apply_montague_recursively(&children[1], terminals_to_fol);
                                let applied_terms = LambdaEntity::Application(Box::new(function_term?), Box::new(argument_term?));
                                return Ok(reduce(&applied_terms))
                            }
                        }
                    }
                    (c, CCGCategory::Composed {left, right, operator }) => {
                        println!("matched FA case 2. operator = {}. {:?} = {:?} and {:?} = {:?}", operator, left, target_category, right, c);

                        if operator == Forward {
                            if *right == c && *left == target_category {
                                let function_term = apply_montague_recursively(&children[1], terminals_to_fol);
                                let argument_term = apply_montague_recursively(&children[0], terminals_to_fol);
                                let applied_terms = LambdaEntity::Application(Box::new(function_term?), Box::new(argument_term?));
                                return Ok(reduce(&applied_terms))
                            }
                        }   
                    }
                    _ => return Err(String::from("FA: One category must be composed"))
                }
                Err(String::from("FA: Target category not found"))
            }
            Some(CCGRule::BA) => {

                println!("doing backward application on {:?}: children {:?} and {:?}\n",  node.category, children[0].category, children[1].category);
                // Backward Application
                let target_category = node.clone().category;

                match (children[0].clone().category, children[1].clone().category) {

                    (CCGCategory::Composed {left, right, operator }, c) => {
                        if operator == Backward {
                            println!("matched BA case 1. operator = {}. {:?} = {:?} and {:?} = {:?}", operator, left, target_category, right, c);
                            if *right == c && *left == target_category {
                                let function_term = apply_montague_recursively(&children[0], terminals_to_fol);
                                let argument_term = apply_montague_recursively(&children[1], terminals_to_fol);
                                let applied_terms = LambdaEntity::Application(Box::new(function_term?), Box::new(argument_term?));
                                return Ok(reduce(&applied_terms))
                            }
                        }
                    }
                    (c, CCGCategory::Composed {left, right, operator }) => {
                        if operator == Backward {
                            println!("matched BA case 2. operator = {}. {:?} = {:?} and {:?} = {:?}", operator, left, target_category, right, c);
                            if *right == c && *left == target_category {
                                let function_term = apply_montague_recursively(&children[1], terminals_to_fol);
                                let argument_term = apply_montague_recursively(&children[0], terminals_to_fol);
                                let applied_terms = LambdaEntity::Application(Box::new(function_term?), Box::new(argument_term?));
                                return Ok(reduce(&applied_terms))
                            }
                        }
                    }
                    _ => return Err(String::from("BA: One category must be composed"))
                }
                Err(String::from("BA: Target category not found"))
            }
            Some(CCGRule::U) => {
                // Unary rule (e.g., type raising)
                let inner_term = apply_montague_recursively(&children[0], terminals_to_fol);
                // Handle unary transformations if necessary (e.g., type raising)
                //println!("HANDLING UNARY: {:?}", inner_term);
                inner_term // Currently no transformation applied
            }
            _ => panic!("Unknown or unsupported CCG rule: {:?}", node.rule),
        }
    }

    // Start recursive application from the root of the CCG tree
    Ok(apply_montague_recursively(ccg_tree, terminals_to_fol)?)
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