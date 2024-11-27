use crate::ccg::ccg_types::{get_terminal_nodes, CCGNode, CCGRule};
use crate::ccg::language_parser::english_to_ccg;
use crate::brill::brill_tagger::tag_sentence;
use crate::brill::contextual_ruleset::parse_contextual_ruleset;
use crate::brill::init_tagger::initialize_tagger;
use crate::brill::lexical_ruleset::parse_lexical_ruleset;
use crate::brill::wordclass::Wordclass;
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

    let sentence = "John likes gouda";

    // retrieve words and their corresponding pos tags
    let vec_of_word_tag_tuples = tag_sentence(sentence, &lexical_ruleset, &contextual_ruleset, &mut wc_mapping);

    // parse the ccg tree
    let ccg = english_to_ccg(sentence, vec_of_word_tag_tuples.clone());

    println!("{}", ccg);

    // map words to their montague grammar representation
    let montague_representation = map_words_to_montague(vec_of_word_tag_tuples.clone(), &ccg);

    println!("{:?}", montague_representation);

    // reduce the montague grammar into a single fol expression
    let fol = reduce_montague(&montague_representation, &ccg);



    println!("REDUCED FOL: {}", fol);
}

/// Reduce individual FOL words into the complete FOL sentence representation using rules from the CCG tree.
fn reduce_montague(terminals_to_fol: &Vec<(String, LambdaEntity)>, ccg_tree: &CCGNode) -> LambdaEntity {
    fn apply_montague_recursively(
        node: &CCGNode,
        terminals_to_fol: &Vec<(String, LambdaEntity)>,
    ) -> LambdaEntity {
        // Base case: If the node is a terminal, return its Montague term
        if node.children == None {
            if let Some(ref text) = node.text {
                // Find the corresponding Montague term for the terminal
                if let Some((_, lambda_entity)) = terminals_to_fol.iter().find(|(word, _)| word == text) {
                    //println!("BASE CASE, TERMINAL: {:?}", lambda_entity.clone());
                    return lambda_entity.clone();
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
            Some(CCGRule::FA) => {
                // Forward Application: function comes first, then argument
                let function_term = apply_montague_recursively(&children[0], terminals_to_fol);
                let argument_term = apply_montague_recursively(&children[1], terminals_to_fol);
                let applied_terms = LambdaEntity::Application(Box::new(function_term), Box::new(argument_term));
                //println!("FA: {:?}", applied_terms);
                reduce(&applied_terms)
            }
            Some(CCGRule::BA) => {
                // Backward Application: argument comes first, then function
                let argument_term = apply_montague_recursively(&children[0], terminals_to_fol);
                let function_term = apply_montague_recursively(&children[1], terminals_to_fol);
                let applied_terms = LambdaEntity::Application(Box::new(function_term), Box::new(argument_term));
                println!("BA: {:?}", applied_terms);
                reduce(&applied_terms)
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
    apply_montague_recursively(ccg_tree, terminals_to_fol)
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