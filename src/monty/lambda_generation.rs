#![allow(non_snake_case)]

use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread::current;
use crate::brill::wordclass::Wordclass;
use crate::ccg::category::CCGType;
use crate::ccg::node::CCGNode;
use crate::ccg::rule::CCGRule;
use crate::lambda::types::*;
use crate::lambda::predicate::Predicate;
use crate::lambda::abstraction::Abstraction;
use crate::lambda::application::Application;
use crate::lambda::conjunction::Conjunction;
use crate::lambda::dependent_function::DependentFunction;
use crate::lambda::variable::Variable;
use crate::lambda::dependent_sum::DependentSum;
use crate::{λAbs, λVar, λApp, λPred, λConj, λDepFun, λDepSum};
use crate::lingo::quantifiers::{UNIVERSAL_QUANTIFIERS, EXISTENTIAL_QUANTIFIERS};
use crate::monty::typing_context::{insert_into_context, TYPING_CONTEXT};

static COUNTER: AtomicUsize = AtomicUsize::new(0);

fn get_next_var() -> String {
    let id = COUNTER.fetch_add(1, Ordering::SeqCst);
    format!("x{}", id)
}


fn generate_lexical_element(node: &CCGNode, root: &CCGNode) -> Box<LambdaEntity> {

    if let Some(ccg_word) = &node.word {

        // Compute a "local" mutated tag (does not affect the original node)
        let effective_tag = if [Wordclass::NN, Wordclass::NNS].contains(&ccg_word.tag)
            && matches!(node.node_type, CCGType::ForwardsFunctor(..) | CCGType::BackwardsFunctor(..))
        {
            Wordclass::VBZ
        } else {
            ccg_word.tag
        };

        match effective_tag {
            // NNP : Proper Nouns are unique (variables)
            Wordclass::NNP => {
                insert_into_context(ccg_word.text.clone(), String::from("Ind"));
                λVar!(ccg_word.text.clone())
            },
            // Common Nouns are types
            Wordclass::NN | Wordclass::NNS => {
                let iden = get_next_var();
                insert_into_context(iden.clone(), ccg_word.text.clone());
                λVar!(iden.clone())
            },
            // Adjectives are predicates
            // john likes tasty cheese -> likes(john, x) ^ tasty(x)
            Wordclass::JJ => {

                let subject = node.get_sibling(root).expect("Expected adjective to have a sibling.");
                let num_args = count_predicate_arguments(node.clone().node_type);
                println!("num args: {num_args}");
                let predicate = generate_predicate(ccg_word.text.clone(), count_predicate_arguments(node.clone().node_type));
                λAbs!(λVar!(String::from("a")), λConj!(predicate, λVar!(String::from("a"))))

            },

            // Wordclass::JJ => generate_predicate(ccg_word.text.clone(), count_predicate_arguments(category.clone())),

            // Verbs are predicates
            Wordclass::VBZ => generate_predicate(ccg_word.text.clone(), count_predicate_arguments(node.clone().node_type)),

            // Since determiners always get forwards/backwards applied to something else, and we want to ignore it here.
            // We can substitute this for some identity function, i.e. \x . x (woman) --b--> woman
            Wordclass::DT => {
                λAbs!(λVar!("ID1".parse().unwrap()), λVar!("ID1".parse().unwrap()))
            },
            _ => panic!("Wordclass variant not implemented: {}", effective_tag),
        }
    } else {
        panic!("expected word and tag on terminal node: {}", node);
    }
}

fn generate_predicate(identifier: String, num_arguments: i32) -> Box<LambdaEntity> {

    if num_arguments == 0 {
        return λVar!(String::from("tmp"))
    }

    let mut arguments = Vec::new();
    for i in 1..=num_arguments {
        arguments.push(λVar!(get_next_var()));
    }

    let mut expression = λPred!(identifier, arguments.clone());
    for arg in arguments.clone() {
        expression = λAbs!(arg, expression);
    }

    expression
}

fn count_predicate_arguments(category: CCGType) -> i32 {
    match category {
        CCGType::ForwardsFunctor(left, right) | CCGType::BackwardsFunctor(left, right)  => {
            1 + count_predicate_arguments(*left) + count_predicate_arguments(*right)
        }
        _ => 0
    }
}

fn unpack_children(maybe_nodes: Option<Vec<Box<CCGNode>>>) -> (CCGNode, CCGNode) {
    let nodes_vec = maybe_nodes.expect("Expected a vector of nodes, found None.");
    let first = nodes_vec.get(0).expect("Expected at least one node, found none.");
    let second = nodes_vec.get(1).expect("Expected at least two nodes, found only one.");
    ( (**first).clone(), (**second).clone() )
}

pub fn ccg_to_lambda(root: &mut CCGNode) -> Box<LambdaEntity> {
    ccg_to_lambda_recursive(root.clone(), root)
}

pub fn ccg_to_quantifier(node: CCGNode, root: &CCGNode) -> Box<LambdaEntity> {
    let bound_var_node = node.get_sibling(root).expect("Expected quantification node to have a sibling");
    let mut bound_var = ccg_to_lambda_recursive(bound_var_node.clone(), root);
    let mut applied_var = bound_var.clone();
    let quantified_phrase = node.get_parent(root).expect("Expected quantification node to have a parent");

    let expr = if let Some(expr_node) = quantified_phrase.backtrack_until_rhs(root) {
        // The quantifier is on the left hand side. e.g. "EVERY MAN, LIKES CHEESE"
        ccg_to_lambda_recursive(expr_node.clone(), root)
    } else if let Some(expr_node) = quantified_phrase.backtrack_until_lhs(root) {
        // The quantifier is on the right hand side. e.g. "JOHN, LIKES EVERY CHEESE"
        let lhs = expr_node.get_sibling(root).expect("Expected quantification node to have a sibling");
        let (left, _) = unpack_children(lhs.clone().children);
        applied_var = ccg_to_lambda_recursive(expr_node.clone(), root);

        λApp!(ccg_to_lambda_recursive(left.clone(), root), bound_var.clone())
    } else {
        // The quantifiers are on both sides. e.g. "EVERY MAN, LIKES SOME CHEESE"
        let right_quantifier = quantified_phrase.get_sibling(root).expect("Expected right quantifier sibling");
        ccg_to_lambda_recursive(right_quantifier.clone(), root)
    };

    match node.word.map(|w| w.text) {
        Some(q) if UNIVERSAL_QUANTIFIERS.contains(&q) => λDepFun!(bound_var.clone(), λApp!(expr, applied_var)),
        Some(q) if EXISTENTIAL_QUANTIFIERS.contains(&q) => λDepSum!(bound_var.clone(), λApp!(expr, applied_var)),
        _ => panic!("Expected quantification node to be existential or universal"),
    }
}


pub fn ccg_to_lambda_recursive(current_node: CCGNode, root: &CCGNode) -> Box<LambdaEntity> {
    use LambdaEntity::*;

    // Handle quantifier nodes
    if current_node.is_quantification_node() {
        return ccg_to_quantifier(current_node.clone(), root);
    }

    match current_node.rule {
        // Base case: terminal nodes
        CCGRule::Lexical => {
            let expr = generate_lexical_element(&current_node, root);
            return expr;
        }

        // Recursive case: Backward Application
        CCGRule::BackwardApplication => {

            let (left, right) = unpack_children(current_node.children);

            // left hand side is quantified expression. right hand side is expr
            // e.g. EVERY MAN, LIKES CHEESE
            if left.contains_quantification_node() && !right.contains_quantification_node() {
                return ccg_to_lambda_recursive(left.clone(), root);
            }
            // right hand side is quantified expression. left hand side is expr
            // e.g. JOHN, LIKES EVERY CHEESE
            if right.contains_quantification_node() && !left.contains_quantification_node() {
                return ccg_to_lambda_recursive(right.clone(), root);
            }

            // both sides are quantified expressions
            // e.g. EVERY MAN, LIKES SOME CHEESE
            if current_node.node_type == CCGType::Sentence && right.contains_quantification_node() && left.contains_quantification_node() && left.clone().node_type != CCGType::Sentence && right.clone().node_type != CCGType::Sentence {
                return ccg_to_lambda_recursive(left.clone(), root);
            }

            return λApp!(
                ccg_to_lambda_recursive(right, root),
                ccg_to_lambda_recursive(left, root)
            );
        }

        // Recursive case: Forward Application
        CCGRule::ForwardApplication => {

            let (left, right) = unpack_children(current_node.children);

            // right hand side is quantified expression. left hand side is expr
            // e.g. JOHN LIKES, EVERY CHEESE
            if left.contains_quantification_node() && !right.contains_quantification_node() {
                return ccg_to_lambda_recursive(left.clone(), root);
            }

            if right.contains_quantification_node() && !left.contains_quantification_node() {
                return ccg_to_lambda_recursive(right.clone(), root);
            }

            return λApp!(
                ccg_to_lambda_recursive(left, root),
                ccg_to_lambda_recursive(right, root)
            );
        }

        // Unary rules must have exactly one child
        CCGRule::Unary => {
            if let Some(children) = &current_node.children {
                if children.len() == 1 {
                    return ccg_to_lambda_recursive(*children[0].clone(), root);
                } else {
                    panic!("Expected one child (unary rule).");
                }
            } else {
                panic!("Expected node to have children.");
            }
        }

        // Conjunction rule
        CCGRule::Conjunction => {
            // Conjunction combines on the right according to CCGBank
            let (left, right) = unpack_children(current_node.children);
            let right_expr = ccg_to_lambda_recursive(right, root);
            let x = λVar!("x".to_string());
            let conj_body = λConj!(x.clone(), right_expr);
            return λAbs!(x, conj_body);
        }

        _ => panic!("Not implemented yet!"),
    }
}