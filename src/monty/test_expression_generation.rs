use std::collections::HashMap;
use crate::lambda::conjunction::Conjunction;
use crate::lambda::predicate::Predicate;
use crate::lambda::variable::Variable;
use crate::lambda::dependent_function::DependentFunction;

use crate::lambda::types::{Expandable, LambdaEntity};
use crate::{λAbs, λApp, λPred, λConj, λVar, λDepFun};
use crate::brill::brill_tagger::tag_sentence;
use crate::brill::contextual_ruleset::parse_contextual_ruleset;
use crate::brill::contextual_rulespec::ContextualRulespec;
use crate::brill::init_tagger::{initialize_tagger, WordclassMap};
use crate::brill::lex_rulespec_id::{LexicalRuleID, LexicalRulespec};
use crate::brill::lexical_ruleset::parse_lexical_ruleset;
use crate::brill::wordclass::Wordclass;
use crate::ccg::sentence_parser::english_to_ccg;
use crate::lambda::reducible::Reducible;
use crate::lambda::types::LambdaEntity::Conj;
use crate::monty::lambda_generation::ccg_to_lambda;

#[test]
fn test_expression_generation() {
    return;
    let (lexical_ruleset, contextual_ruleset, mut wc_mapping) = (
        parse_lexical_ruleset("data/rulefile_lexical.txt").unwrap(),
        parse_contextual_ruleset("data/rulefile_contextual.txt").unwrap(),
        initialize_tagger("data/lexicon.txt").unwrap()
    );

    let test_cases = vec![
        (
            "man walks",
            λPred!("walks".to_string(), vec![λVar!("man".to_string())])
        ),
        (
            "man runs and eats cake",
            λConj!(
                λPred!("runs".to_string(), vec![λVar!("man".to_string())]),
                λPred!("eats".to_string(), vec![λVar!("man".to_string()), λVar!("cake".to_string())])
            )
        ),
        (
            "man walks and runs and eats cake",
            λConj!(
                λConj!(
                    λPred!("walks".to_string(), vec![λVar!("man".to_string())]),
                    λPred!("runs".to_string(), vec![λVar!("man".to_string())])
                ),
                λPred!("eats".to_string(), vec![λVar!("man".to_string()), λVar!("cake".to_string())])
            )
        ),
        (
            "every man and every woman and every child likes brie",
            λConj!(
                λDepFun!(λVar!("man".to_string()), λPred!("likes".to_string(), vec![λVar!("man".to_string()), λVar!("brie".to_string())])),
                λConj!(
                    λDepFun!(λVar!("woman".to_string()), λPred!("likes".to_string(), vec![λVar!("woman".to_string()), λVar!("brie".to_string())])),
                    λDepFun!(λVar!("child".to_string()), λPred!("likes".to_string(), vec![λVar!("child".to_string()), λVar!("brie".to_string())]))
                )
            )
        )
    ];

    for (sentence, expected_output) in test_cases {
        let output = generate_expression(sentence, &lexical_ruleset, &contextual_ruleset, &mut wc_mapping);
        assert_eq!(output, expected_output);
    }
}

fn generate_expression(sentence: &str, lexical_ruleset: &Vec<LexicalRulespec>, contextual_ruleset: &HashMap<Wordclass, Vec<ContextualRulespec>>, wc_mapping: &mut WordclassMap) -> Box<LambdaEntity> {
    let tagged_words = tag_sentence(sentence, lexical_ruleset, contextual_ruleset, wc_mapping);
    let mut ccg = english_to_ccg(sentence, tagged_words);
    Box::from(ccg_to_lambda(&mut ccg).beta_reduce().expand())
}
