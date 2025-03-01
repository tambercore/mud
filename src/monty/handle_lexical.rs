use crate::ccg::category::CCGType;
use crate::ccg::node::CCGNode;
use crate::ccg::rule::CCGRule;
use crate::lambda::types::LambdaEntity;
use crate::lambda::abstraction::Abstraction;
use crate::lambda::application::Application;
use crate::lambda::variable::Variable;
use crate::lambda::predicate::Predicate;
use crate::lambda::casef::CaseHandler;
use crate::{λAbs, λVar, λPred, λCaseF, λApp};
use crate::lingo::quantifiers::EXISTENTIAL_QUANTIFIERS;
use crate::monty::fresh_variable::{fresh_variable, reset_counter, to_unicode_subscript};



pub fn gen_predicate(word: String, _typ: CCGType, depth: i64 ) -> Box<LambdaEntity> {
    match _typ {
        /* Functor Types (Inductive Case) */
        CCGType::ForwardsFunctor(l, r) |
        CCGType::BackwardsFunctor(r, l) => { λAbs! ( λVar!(fresh_variable()), gen_predicate(word, *l, depth + 1) ) }

        /* Non Functor Type (Base Case) */
        _ => {
            let args: Vec<Box<LambdaEntity>> = (1..=depth as usize).rev()
                .map(|i| { λVar! (format!("x{}", to_unicode_subscript(i)))}).collect();
            λPred!(word, args)
        }
    }
}



pub fn lexical_to_lambda(node: CCGNode) -> Box<LambdaEntity> {

    /* Ensure the node is Lexical, this function shouldn't be called on a non-lexical node. */
    if node.rule != CCGRule::Lexical { panic!("Function `lexical_to_lambda` called on Non-Lexical Node"); }

    /* Extract the word itself 🍥*/
    let mut word = node.word.clone().expect("Lexical to Lambda expects CCGNode to contain a `word`").text;

    use CCGType::*;
    match node.clone().node_type {

        /* Noun/NounPhrase is simply a variable, this will be eventually bound */
        Noun | NounPhrase => { return λVar!(word.to_string()) }

        /* Functor Types should bind variable into predicates through an abstraction */
        ForwardsFunctor(l, r) |
            BackwardsFunctor(r, l) => {

            /* Handle `is` */
            if (&node.clone().word.unwrap().text == "is") {
                return λCaseF!(
                    λAbs!(λVar!(String::from("x₁")), λAbs!(λVar!(String::from("x₂")), λApp!(λVar!(String::from("x₁")), λVar!(String::from("x₂"))))),
                    λAbs!(λVar!(String::from("x₁")), λAbs!(λVar!(String::from("x₂")), λPred!(String::from("is"), vec![λVar!(String::from("x₂")), λVar!(String::from("x₁"))])))
                )
            }

            /* Handle existential quantifiers i.e. 'a' 'some' as identity functions */
            if EXISTENTIAL_QUANTIFIERS.contains(&node.clone().word.unwrap().text) {
                return λAbs! ( λVar!((format!("i{}", to_unicode_subscript(0)))),
                               λVar!((format!("i{}", to_unicode_subscript(0)))));
            }

            /* Handle normal predicates, as normally */
            reset_counter();
            return gen_predicate(word, node.clone().node_type, 0)
        }

        _ => { panic!("Not yet implemented!") }
    }
}
