use crate::ccg::category::CCGType;
use crate::ccg::node::CCGNode;
use crate::ccg::rule::CCGRule;
use crate::lambda::types::LambdaEntity;
use crate::lambda::abstraction::Abstraction;
use crate::lambda::variable::Variable;
use crate::lambda::predicate::Predicate;
use crate::{λAbs, λVar, λPred};
use crate::monty::fresh_variable::{fresh_variable, to_unicode_subscript};

pub fn tf (word: String, _typ: CCGType, depth: i64 ) -> Box<LambdaEntity> {

    use CCGType::*;
    match _typ {
        ForwardsFunctor(l, r) => { λAbs! ( λVar!(fresh_variable()), tf(word, *l, depth + 1) ) }
        BackwardsFunctor(l, r) => { λAbs! ( λVar!(fresh_variable()), tf(word, *r, depth + 1) ) }

        _ => {
            let args: Vec<Box<LambdaEntity>> = (1..=depth as usize)
                .rev()
                .map(|i| {
                    λVar! (format!("x{}", to_unicode_subscript(i)))
                })
                .collect();
            λPred!(word, args)
        }
    }
}

pub fn lexical_to_lambda(node: CCGNode) -> Box<LambdaEntity> {

    /* Ensure the node is Lexical, this function shouldn't be called on a non-lexical node. */
    if node.rule != CCGRule::Lexical { panic!("Function `lexical_to_lambda` called on Non-Lexical Node"); }

    /* Extract the word itself 🍥*/
    let mut word = node.word.clone().expect("Lexical to Lambda expects CCGNode to contain a `word`");

    use CCGType::*;
    match node.clone().node_type {

        /* Noun/NounPhrase is simply a variable, this will be eventually bound */
        Noun | NounPhrase => { return λVar!(word.text.to_string()) }

        /* Functor Types should bind variable into predicates through an abstraction */
        ForwardsFunctor(l, r) |
            BackwardsFunctor(r, l) => {
            return tf(word.text, node.clone().node_type, 0)
        }

        Conjunction => {}
        ConjunctionTag => {}
        PrepositionalPhrase => {}
        Punctuation => {}
        Sentence => {}
        Empty => {}
    }
    
    λVar!("X".to_string())
}

