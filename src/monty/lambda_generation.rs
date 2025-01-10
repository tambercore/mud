#![allow(non_snake_case)]

use std::cell::RefCell;
use std::rc::Rc;

use crate::brill::wordclass::Wordclass;
use crate::brill::init_tagger::{initialize_tagger, WordclassMap};
use crate::brill::brill_tagger::get_possible_tags;
use crate::ccg::category::CCGType;
use crate::ccg::node::CCGNode;
use crate::ccg::rule::CCGRule;
use crate::lambda::types::*;
use crate::lambda::predicate::Predicate;
use crate::lambda::abstraction::Abstraction;
use crate::lambda::application::Application;
use crate::lambda::conjunction::Conjunction;
use crate::lambda::variable::Variable;
use crate::{λAbs, λVar, λApp, λPred, λConj};

/// For a given node type and node, build a lexical category (Box<LambdaEntity>).
fn generate_lexical_category(_type: CCGType, _node: &CCGNode) -> Box<LambdaEntity> {
    match _type {
        CCGType::ForwardsFunctor(left, right) | CCGType::BackwardsFunctor(left, right) => {
            // Recursively generate a lambda abstraction for functor categories
            let lexical_category = λAbs!(
                generate_lexical_category(*right, _node),
                generate_lexical_category(*left, _node)
            );
            generate_lexical_element(_node, lexical_category)
        }
        // If it's a simple (non-functor) category, just wrap it in a variable
        _ => generate_lexical_element(_node, λVar!(_type.to_string())),
    }
}

/// Generate a lexical lambda entity (predicate or variable) based on a node’s word/tag.
fn generate_lexical_element(node: &CCGNode, category: Box<LambdaEntity>) -> Box<LambdaEntity> {
    if let Some(ccg_word) = &node.word {
        // If the node is a functor but the tag is NN or NNS, we might treat it as a VBZ
        let effective_tag = if [Wordclass::NN, Wordclass::NNS].contains(&ccg_word.tag)
            && matches!(node.node_type, CCGType::ForwardsFunctor(..) | CCGType::BackwardsFunctor(..))
        {
            Wordclass::VBZ
        } else {
            ccg_word.tag
        };

        match effective_tag {
            Wordclass::NNP | Wordclass::NN => λVar!(ccg_word.text.clone()),
            Wordclass::VBZ => generate_predicate(ccg_word.text.clone(), category),
            _ => panic!("wordclass variant not implemented: {:?}", effective_tag),
        }
    } else {
        panic!("Expected a word/tag on a terminal node: {}", node);
    }
}

/// Builds a lambda expression for a predicate with the given identifier.
fn generate_predicate(identifier: String, category: Box<LambdaEntity>) -> Box<LambdaEntity> {
    let num_arguments = count_predicate_arguments(category.clone());

    // A small safeguard for unexpected zero-argument predicates
    if num_arguments == 0 {
        return λVar!(String::from("tmp"));
    }

    // Build argument list: x1, x2, ...
    let mut arguments = Vec::new();
    for i in 1..=num_arguments {
        arguments.push(λVar!(format!("x{}", i)));
    }

    // Start with λPred!(identifier, [x1, x2, ...])
    let mut expression = λPred!(identifier, arguments);

    // Then wrap it in λAbs! for each argument in reverse order
    for i in (1..=num_arguments).rev() {
        let arg_name = format!("x{}", i);
        expression = λAbs!(λVar!(arg_name), expression);
    }

    expression
}

/// Count how many arguments a functor category expects (by counting nested lambda Abs nodes).
fn count_predicate_arguments(category: Box<LambdaEntity>) -> i32 {
    match *category {
        LambdaEntity::Abs(abs) => {
            // Every Abs node might introduce an argument,
            // but we also count inside its bound_var and body if they're Abs, too
            1 + count_predicate_arguments(abs.bound_var) + count_predicate_arguments(abs.body)
        }
        LambdaEntity::Var(_) | LambdaEntity::Pred(_) => 0,
        _ => panic!("Invalid application in lexical term"),
    }
}

/// Convert a `CCGNode` (with rule + children) into a `LambdaEntity` expression.
pub fn ccg_to_lambda(root: CCGNode) -> Box<LambdaEntity> {
    match root.rule {
        // --- Base case: lexical/terminal nodes ---
        CCGRule::Lexical => generate_lexical_category(root.node_type.clone(), &root),

        // --- Recursive cases ---
        CCGRule::BackwardApplication => {
            let children_borrowed = root.children.borrow();
            if children_borrowed.len() < 2 {
                panic!("Expected at least two children for BackwardApplication");
            }
            let left  = children_borrowed[0].borrow().clone();
            let right = children_borrowed[1].borrow().clone();
            λApp!(ccg_to_lambda(right), ccg_to_lambda(left))
        }
        CCGRule::ForwardApplication => {
            let children_borrowed = root.children.borrow();
            if children_borrowed.len() < 2 {
                panic!("Expected at least two children for ForwardApplication");
            }
            let left  = children_borrowed[0].borrow().clone();
            let right = children_borrowed[1].borrow().clone();
            λApp!(ccg_to_lambda(left), ccg_to_lambda(right))
        }
        CCGRule::Unary => {
            let children_borrowed = root.children.borrow();
            if children_borrowed.len() == 1 {
                let child = children_borrowed[0].borrow().clone();
                ccg_to_lambda(child)
            } else {
                panic!("Expected exactly one child for a Unary rule, found {}", children_borrowed.len());
            }
        }
        CCGRule::Conjunction => {
            let children_borrowed = root.children.borrow();
            if children_borrowed.len() == 2 {
                let lhs_type = children_borrowed[0].borrow().node_type.clone();
                let rhs_type = children_borrowed[1].borrow().node_type.clone();

                match (lhs_type, rhs_type) {
                    // If the first child is the actual conjunction "and"
                    (CCGType::Conjunction, _) => {
                        λAbs!(
                            λVar!("x1".to_string()),
                            λConj!(
                                λVar!("x1".to_string()),
                                ccg_to_lambda(children_borrowed[1].borrow().clone())
                            )
                        )
                    }
                    // If the second child is the actual conjunction "and"
                    (_, CCGType::Conjunction) => {
                        λAbs!(
                            λVar!("x1".to_string()),
                            λConj!(
                                ccg_to_lambda(children_borrowed[0].borrow().clone()),
                                λVar!("x1".to_string())
                            )
                        )
                    }
                    _ => panic!("Expecting at least one child to have type Conjunction (CCGType::Conjunction)"),
                }
            } else {
                panic!("Expected exactly 2 children for Conjunction rule, found {}", children_borrowed.len());
            }
        }

        // --- Anything else is not implemented in your match ---
        _ => panic!("Not implemented yet!"),
    }
}