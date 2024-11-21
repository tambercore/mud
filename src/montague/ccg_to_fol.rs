use crate::ccg::ccg_types::CCGNode;
use crate::ccg::language_parser::english_to_ccg;
use crate::brill::brill_tagger::tag_sentence;
use crate::brill::contextual_ruleset::parse_contextual_ruleset;
use crate::brill::init_tagger::initialize_tagger;
use crate::brill::lexical_ruleset::parse_lexical_ruleset;

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

    let sentence = "John likes cheese"; // Likes(John, Cheese)

    // retrieve words and their corresponding pos tags
    let vec_of_word_tag_tuples = tag_sentence(sentence, &lexical_ruleset, &contextual_ruleset, &mut wc_mapping);

    let ccg = english_to_ccg(sentence, vec_of_word_tag_tuples);
    println!("{}", ccg);
    // let fol = ccg_to_fol(ccg);
}