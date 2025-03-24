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
mod ast;

use crate::ast::program::{initialise_agda_file, Program};
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicBool, Ordering};
use serde::{Deserialize, Serialize};
use warp::Filter;
use crate::brill::brill_tagger::{get_sentence_tags, tag_sentence};
use crate::brill::contextual_ruleset::parse_contextual_ruleset;
use crate::brill::init_tagger::{initialize_tagger, WordclassMap};
use crate::brill::lexical_ruleset::parse_lexical_ruleset;
use crate::brill::utils::{create_tag_mapping, TAG_MAPPING};
use crate::monty::ccg_to_lc::*;
use crate::lambda::reducible::*;
use crate::lambda::types::{Expandable, LambdaEntity};
use crate::composer::lambda_to_types::compose;
use crate::command_line::get_arguments::{Config};
use crate::composer::knowledge_base::{compose_kb, KnowledgeBase};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use attohttpc::header::SERVER;
use colored::Colorize;
use crossterm::style::Stylize;
use crate::ast::agda_expr::AgdaExpr;
use crate::brill::contextual_rulespec::ContextualRulespec;
use crate::brill::lex_rulespec_id::LexicalRulespec;
use crate::brill::wordclass::Wordclass;
use crate::ccg::lambeq_parser::{sentences_to_ccg_hashsets, SENTENCE_TO_CCG, SENTENCE_TO_JSON};
use crate::ccg::sentence_parser::english_to_ccg;
use crate::command_line::output_handler::{create_task, show_header, update_task};
use crate::composer::conclusions::compose_conclusions;
use crate::composer::langtree::{lambda_to_semantic, SemanticTree};
use crate::lambda::etalike::Eliminator;
use crate::resolver::fill_holes::fill_holes;
use crate::server::server::{create_endpoint, AgdaConclusion, AgdaPremise};
use crate::wordnet::interface::init_wordnet;

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

static SERVER_RUNNING: AtomicBool = AtomicBool::new(false);
static WORDS_IN_EXISTENCE: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| {
    Mutex::new(HashSet::new())
});

fn sentence_to_agda(sentence: String, f: &mut Program) -> ((String, AgdaExpr), String) {

    show_header(&format!("Processing Sentence '{}'", sentence));
    let brill_task_id = create_task(1, "Assigning POS Tags w/ Brill");

    /* Access the global references for the brill tagger! */
    let lexical_ruleset = &*LEXICAL_RULESET;
    let contextual_ruleset = &*CONTEXTUAL_RULESET;
    let mut wc_mapping = WC_MAPPING.lock().unwrap();

    let possible_tags = get_sentence_tags(&sentence, &mut wc_mapping);
    let vec_of_word_tag_tuples = tag_sentence(&sentence, lexical_ruleset, contextual_ruleset, &mut wc_mapping);
    create_tag_mapping(possible_tags, vec_of_word_tag_tuples.clone());

    update_task(brill_task_id);

    let lc_task_id = create_task(1, "Converting CCG to λ-Calculus");
    // println!("tag mapping: {:?}", TAG_MAPPING.get().unwrap());

    let (mut ccg, json_tree) = if SERVER_RUNNING.load(Ordering::SeqCst) {
        let ccg = SENTENCE_TO_CCG.read().unwrap().iter().find(|(s, _)| *s == sentence).map(|(_, ccg)| ccg.clone()).expect("Failed to map sentence to ccg.");
        let json_tree = SENTENCE_TO_JSON.read().unwrap().iter().find(|(s, _)| *s == sentence).map(|(_, json)| json.clone()).expect("Failed to map sentence to json.");
        (ccg, json_tree)
    } else {
        english_to_ccg(&sentence, vec_of_word_tag_tuples.clone())
    };
    println!("CCG: {}", ccg);

    let lambda_expression = ccg_to_lambda(&mut ccg);
    //println!("Result: \n{}", lambda_expression);
    update_task(lc_task_id);

    println!("Lambda Expr: {lambda_expression}");

    let lc_task_id2 = create_task(1, "β-Reduction, η-Reduction & Expansions");
    let reduction = (*lambda_expression).beta_reduce();
    //println!("\n\nReduces to: \n{}", reduction);

    println!("Lambda Expr pre-eta: {reduction}");

    let eta_reduction = (reduction).eliminate_leftovers();
    //println!("\n\nEta Reduces to: \n{}", eta_reduction);

    println!("Reduced Expr: {eta_reduction}");

    let expansion = eta_reduction.expand();

    println!("Expanded Expr: {expansion}");

    let expanded_expression: Box<LambdaEntity> = Box::from(expansion);
    //println!("\n\nExpands to: {}", expanded_expression);
    update_task(lc_task_id2);

    println!("Expanded Expr: {expanded_expression}");

    let semtree_id = create_task(1, "Converting to Semantic Tree");
    let semantic_tree = lambda_to_semantic(Box::from(expanded_expression.clone())).expect("Failed to parse semantic tree.");

    println!("Semantic Tree: {}", semantic_tree);

    update_task(semtree_id);


    let agda_id = create_task(1, "Generating Agda Constructs");
    let encoded_sentence = compose(Box::from(semantic_tree), f, vec![]);
    update_task(agda_id);
    println!("");

    show_header(&format!("Displaying CCG Derivation for '{}'", sentence));
    println!("{}", ccg);

    (encoded_sentence, json_tree)
}



fn english_to_agda(knowledge: Vec<String>, derivations: Vec<String>) -> (Program, Vec<AgdaPremise>, Vec<AgdaConclusion>) {

    println!();
    print!("\x1b[38;5;130m[mud]\x1b[0m \x1b[1m{}\x1b[0m", "");
    print!(" Welcome to the \x1b[38;5;130mMud Theorem Prover\x1b[0m");
    println!("!");
    println!(" ⋅ It seems you have given me some {}, and {},", Colorize::purple("Premises"), Colorize::blue("Conclusions"));
    println!(" ⋅ I will try my best to construct this as a proof in {}.", (Colorize::underline(Colorize::cyan("Agda"))));
    println!(" ⋅ Here's exactly what I am going to try and prove.");

    for (i, item) in knowledge.iter().enumerate() {
        println!(" ⋅ {}   →   '{}'", (Colorize::purple(format!("Premise {}   ", i+1).as_str())), Colorize::italic(item.as_str()));
    }

    for (i, item) in derivations.iter().enumerate() {
        println!(" ⋅ {}   →   '{}'", (Colorize::blue(format!("Hypothesis {}", i+1).as_str())), Colorize::italic(item.as_str()));
    }
    println!();

    show_header("Initializing Dependencies & Preprocessing");

    /* Initialize Wordnet */
    let wordnet_task_id = create_task(1, "Initializing Wordnet.");
    // init_wordnet();
    update_task(wordnet_task_id);

    let global_wordset_task_id = create_task(1, "Computing Global Wordset.");
    /* Compute and update the global words in existence */
    let mut knowledge_mut = knowledge.clone();
    knowledge_mut.extend(derivations.clone());

    /* Wrap in a codeblock to release deadlock on global_words */
    {
        /* todo: Update this to use the post-contraction versions */
        let new_words_in_existence: HashSet<String> = knowledge_mut
            .iter()
            .flat_map(|s| s.split_whitespace())
            .map(String::from)
            .collect();

        /* Lock the mutex and update the set */
        let mut global_words = WORDS_IN_EXISTENCE.lock().unwrap();
        *global_words = new_words_in_existence;
    }
    update_task(global_wordset_task_id);


    /* Initialise the CCG and JSON representations of each sentence. */
    let ccg_task_id = create_task(1, "Initializing CCG Trees with Lambeq.");
    let sentences: Vec<String> = knowledge.clone().into_iter().chain(derivations.clone().into_iter()).collect();
    if let Err(err) = sentences_to_ccg_hashsets(sentences) {
        // eprintln!("Failed to parse sentences into CCG: {}", err);
        SERVER_RUNNING.store(false, Ordering::SeqCst);
    } else {
        SERVER_RUNNING.store(true, Ordering::SeqCst);
    }
    update_task(ccg_task_id);
    println!("");

    /* Initialise the Agda File (get it ready) */
    let mut f = initialise_agda_file();

    /* Initialise an empty vector to hold each premise and conclusion in JSON form. */
    let mut premises : Vec<AgdaPremise> = Vec::new();
    let mut conclusions: Vec<AgdaConclusion> = Vec::new();

    /* Handle Assumptions */
    let mut encoded_knowledge: KnowledgeBase = vec![];
    for sentence in knowledge {
        let (encoded_sentence, ccg_json) = sentence_to_agda(sentence.clone(), &mut f);
        encoded_knowledge.push(encoded_sentence);

        /* Collect information about premises into a struct. */
        let premise = AgdaPremise {text : sentence.clone(), ccg_tree : ccg_json};
        premises.push(premise);
    }

    /* Handle Conclusions */
    let mut encoded_conclusions: Vec<(String, AgdaExpr)> = vec![];
    for derivation in derivations {
        let (encoded_conclusion, ccg_json) = sentence_to_agda(derivation.clone(), &mut f);
        encoded_conclusions.push(encoded_conclusion);

        /* Collect information about conclusions into a struct. */
        let conclusion = AgdaConclusion {text : derivation.clone(), ccg_tree : ccg_json, filled : false};
        conclusions.push(conclusion);
    }

    show_header("Generating Agda Code & Writing File");
    let kb_task = create_task(1, "Composing Premises to Agda (KB).");
    compose_kb(encoded_knowledge, &mut f);
    update_task(kb_task);

    let cc_task = create_task(1, "Composing Conclusions to Agda.");
    compose_conclusions(encoded_conclusions, &mut f);
    update_task(cc_task);

    (f, premises, conclusions)
}



#[tokio::main]
async fn main() {
    let config = Config::from_args("every man is mortal & socrates is a man -> socrates is mortal");
    let knowledge = config.knowledge;
    let conclusions = config.conclusions;

    if config.server {
        create_endpoint(config.output_file).await;
    } else {
        let (mut agda_file, premises, mut conclusions) = english_to_agda(knowledge.clone(), conclusions.clone());

        let write_tsk = create_task(1, "Writing to Agda File.");
        agda_file.write_to_file(config.output_file.clone());
        update_task(write_tsk);

        let hole_tsk = create_task(1, "Synthesise Holes with Agsy.");
        fill_holes(config.output_file.clone(), &mut conclusions);
        update_task(hole_tsk);

        // println!("\n\nconclusions: {:?}", conclusions);
        SERVER_RUNNING.store(false, Ordering::SeqCst); // Ensure it's false if running locally
    }
}
