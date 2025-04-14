use crate::ccg::category::CCGType;
use crate::ccg::node::CCGNode;
use crate::ccg::rule::CCGRule;
use crate::lambda::types::LambdaEntity;
use crate::lambda::abstraction::Abstraction;
use crate::lambda::variable::Variable;
use crate::lambda::predicate::Predicate;
use crate::{λAbs, λVar, λPred};
use crate::lingo::quantifiers::EXISTENTIAL_QUANTIFIERS;
use crate::monty::fresh_variable::{fresh_variable, reset_counter, to_unicode_subscript};

/// Function to generate a λ-Predicate from a word and CCG type.
pub fn convert_compound_syntactic_type(word: String, _typ: CCGType, depth: i64) -> Box<LambdaEntity> {
    match _typ {
        /* Functor Types (Inductive Case) */
        CCGType::ForwardsFunctor(l, r) | CCGType::BackwardsFunctor(r, l) => {
            λAbs!(λVar!(fresh_variable()), convert_compound_syntactic_type(word, *l, depth + 1))
        }

        /* Non-Functor Type (Base Case) */
        _ => {
            /* Generate variable arguments for the predicate */
            let args: Vec<Box<LambdaEntity>> = (1..=depth as usize)
                .rev()
                .map(|i| λVar!(format!("x{}", to_unicode_subscript(i))))
                .collect();
            λPred!(word, args)
        }
    }
}



/// Function to convert a lexical CCG node into a λ-term.
pub fn convert_syntactic_type(node: CCGNode) -> Box<LambdaEntity> {
    /* Ensure the node is Lexical, this function shouldn't be called on a non-lexical node. */
    if node.rule != CCGRule::Lexical {
        panic!("Function `lexical_to_lambda` called on Non-Lexical Node");
    }

    /* Extract the word itself */
    let mut word = node
        .word
        .clone()
        .expect("Lexical to Lambda expects CCGNode to contain a `word`")
        .text;

    use CCGType::*;
    match node.clone().node_type {
        /* Noun/NounPhrase is simply a variable, this will be eventually bound */
        Noun | NounPhrase => λVar!(word.to_string()),

        /* Functor Types should bind variable into predicates through an abstraction */
        ForwardsFunctor(l, r) | BackwardsFunctor(r, l) => {

            /* Special handling for negation */
            if node.clone().word.unwrap().text == "not" {
                return λAbs!(
                    λVar!(String::from("n₁")),
                    λPred!(String::from("not"), vec![λVar!(String::from("n₁"))])
                );
            }

            /* Special case for to */
            if node.clone().word.unwrap().text == "to" {
                return λAbs!(
                    λVar!(String::from("I")),
                    λVar!(String::from("I"))
                );
            }

            /* Handle existential quantifiers i.e. 'a', 'some' as identity functions */
            if EXISTENTIAL_QUANTIFIERS.contains(&node.clone().word.unwrap().text) {
                return λAbs!(
                    λVar!(format!("i{}", to_unicode_subscript(0))),
                    λVar!(format!("i{}", to_unicode_subscript(0)))
                );
            }

            /* Handle normal predicates as usual */
            reset_counter();
            convert_compound_syntactic_type(word, node.clone().node_type, 0)
        }

        _ => panic!("Not yet implemented!"),
    }
}
