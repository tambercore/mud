#![allow(non_snake_case)]

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



fn generate_lexical_category(_type: CCGType, _node: &CCGNode, root: &CCGNode) -> Box<LambdaEntity> {
    match _type {
        CCGType::ForwardsFunctor(left, right) | CCGType::BackwardsFunctor(left, right) => {
            let lexical_category = λAbs!(generate_lexical_category(*right, _node, root), generate_lexical_category(*left, _node, root));
            generate_lexical_element(_node, lexical_category, root)
        }
        _ => generate_lexical_element(_node, λVar!(_type.to_string()), root)
    }
}

fn generate_lexical_element(node: &CCGNode, category: Box<LambdaEntity>, root: &CCGNode) -> Box<LambdaEntity> {

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
                Wordclass::NNP | Wordclass::NN | Wordclass::NNS => λVar!(ccg_word.text.clone()),
                Wordclass::VBZ => generate_predicate(ccg_word.text.clone(), category),

                // Since determiners always get forwards/backwards applied to something else, and we want to ignore it here.
                // We can substitute this for some identity function, i.e. \x . x (woman) --b--> woman
                Wordclass::DT => λAbs!(λVar!("ID1".parse().unwrap()), λVar!("ID1".parse().unwrap())),
                _ => panic!("Wordclass variant not implemented: {}", effective_tag),
            }
    } else {
        panic!("expected word and tag on terminal node: {}", node);
    }
}

fn generate_predicate(identifier: String, category: Box<LambdaEntity>) -> Box<LambdaEntity> {
    let num_arguments = count_predicate_arguments(category.clone());

    if num_arguments == 0 {
        return λVar!(String::from("tmp"))
    }

    let mut arguments = Vec::new();
    for i in 1..=num_arguments {
        arguments.push(λVar!(format!("x{}", i)));
    }

    let mut expression = λPred!(identifier, arguments);
    for i in 1..=num_arguments {
        let arg_name = format!("x{}", i);
        expression = λAbs!(λVar!(arg_name), expression);
    }

    expression
}

fn count_predicate_arguments(category: Box<LambdaEntity>) -> i32 {
    match *category {
        LambdaEntity::Abs(abs) => {
            1 + count_predicate_arguments(abs.bound_var) + count_predicate_arguments(abs.body)
        }
        LambdaEntity::Var(_) => 0,
        LambdaEntity::Pred(_) => 0,
        _ => panic!("Invalid application in lexical term"),
    }
}

fn unpack_children(maybe_nodes: Option<Vec<Box<CCGNode>>>) -> (CCGNode, CCGNode) {
    let nodes_vec = maybe_nodes.expect("Expected a vector of nodes, found None.");
    let first = nodes_vec.get(0).expect("Expected at least one node, found none.");
    let second = nodes_vec.get(1).expect("Expected at least two nodes, found only one.");
    ( (**first).clone(), (**second).clone() )
}

pub fn ccg_to_lambda(root: &mut CCGNode) -> Box<LambdaEntity> {
    root.initialize_flags();
    ccg_to_lambda_recursive(root.clone(), root)
}

pub fn ccg_to_quantifier(node: CCGNode, root: &CCGNode) -> Box<LambdaEntity> {
    let bound_var_node = node.get_sibling(root).expect("Expected quantification node to have a sibling");
    let bound_var = ccg_to_lambda_recursive(bound_var_node.clone(), root);
    let expr_node = node.backtrack_until_rhs(root).expect("Expected expression in quantification node");
    let expr = ccg_to_lambda_recursive(expr_node.clone(), root);

    println!("quantifier: {} bound variable: {} expr: {}", node, bound_var, expr);

    if node.is_universal_quantification_node {
        //let bound_var = build_bound_variable(bound_var_node.clone(), root, UNIVERSAL_QUANTIFIERS.to_owned());

        λDepFun!(bound_var.clone(), λApp!(expr, bound_var))
    }
    else if node.is_existential_quantification_node {
        //let bound_var = build_bound_variable(bound_var_node.clone(), root, EXISTENTIAL_QUANTIFIERS.to_owned());
        λDepSum!(bound_var.clone(), λApp!(expr, bound_var))
    }
    else {panic!("Expected quantification node to be existential or universal")}
}


fn build_bound_variable(bound_var: CCGNode, root: &CCGNode, quantifiers: Vec<String>) -> Box<LambdaEntity> {

    // todo: alter this to work with the new quantification node
    if let Some(children) = bound_var.children.clone() {
        if let (quant, bound_var) = (&children[0].clone(), &children[1].clone()) {
            let reduced_bound = ccg_to_lambda_recursive(*bound_var.clone(), root);

            if let Some(word) = quant.clone().word {
                if quantifiers.contains(&word.text.to_lowercase()) {
                    return reduced_bound;
                }
            }
            return λApp!(reduced_bound, build_bound_variable(*quant.clone(), root, quantifiers));
        } else {
            panic!("Expected quantification node to have two children")
        }
    } else {
        panic!("Expected quantification node to have children")
    }
}


// TODO: this was separated because root is passed in. Can be removed when a reference to the parent is stored.
pub fn ccg_to_lambda_recursive(current_node: CCGNode, root: &CCGNode) -> Box<LambdaEntity> {
    use LambdaEntity::*;

    if current_node.is_universal_quantification_node | current_node.is_existential_quantification_node {
        ccg_to_quantifier(current_node.clone(), root)
    }

    else {
        match current_node.rule {
            // Base case: terminal nodes
            CCGRule::Lexical => {
                let expr = generate_lexical_category(current_node.node_type.clone(), &current_node, root);
                expr
            },

            // Recursive case
            CCGRule::BackwardApplication => {
                let (left, right) = unpack_children(current_node.children);
                λApp!(ccg_to_lambda_recursive(right, root), ccg_to_lambda_recursive(left, root))
            },

            CCGRule::ForwardApplication => {
                let (left, right) = unpack_children(current_node.children);
                λApp!(ccg_to_lambda_recursive(left, root), ccg_to_lambda_recursive(right, root))
            }

            CCGRule::Unary => {
                if let Some(children) = &current_node.children {
                    if children.len() == 1 {
                        ccg_to_lambda_recursive(*children[0].clone(), root)
                    } else {
                        panic!("Expected one child (unary rule).");
                    }
                } else { panic!("Expected node to have children.") }
            }

            CCGRule::Conjunction => {
                // conjunction combines on the right, according to ccgbank
                let (left, right) = unpack_children(current_node.children);
                let right_expr = ccg_to_lambda_recursive(right, root);
                let x = λVar!("x".to_string());
                let conj_body = λConj!(
                    x.clone(),
                    right_expr
                );
                λAbs!(x, conj_body)
            }


            _ => panic!("Not implemented yet!")
        }
    }
}
