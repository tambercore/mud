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
use crate::{λAbs, λVar, λApp, λPred, λConj, λDepFun};
use crate::brill::brill_tagger::get_possible_tags;
use crate::brill::init_tagger::{initialize_tagger, WordclassMap};

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
    println!("generate lexical element: currently considering {}", node);

    if let Some(ccg_word) = &node.word {

        // If the word is every, construct a dependent function type
        if ccg_word.text.to_lowercase() == "every" {
            println!("root in dep fun: {}", root.clone());

            // Retrieve the parent of the current node
            let parent = node.get_parent(&root);
            println!("parent of {} : {:?}", ccg_word.text, parent);

            let sibling = node.get_sibling(&root);
            println!("sibling of {} : {:?}", ccg_word.text, sibling);

            if let (Some(p), Some(x)) = (parent, sibling) {
                // Retrieve the sibling of the parent
                let y = p.get_sibling(&root);

                if let Some(y) = y {
                    let reduced_x = ccg_to_lambda_recursive(x.clone(), root);
                    let reduced_y = ccg_to_lambda_recursive(y.clone(), root);

                    let ret = λDepFun!(reduced_x, reduced_y);

                    println!("returning {:?}", ret);
                    ret
                } else {
                    panic!("Parent has no sibling.");
                }
            } else {
                panic!("Node is missing a parent or sibling.");
            }

        } else {
            // Compute a "local" mutated tag (does not affect the original node)
            let effective_tag = if [Wordclass::NN, Wordclass::NNS].contains(&ccg_word.tag)
                && matches!(node.node_type, CCGType::ForwardsFunctor(..) | CCGType::BackwardsFunctor(..))
            {
                Wordclass::VBZ
            } else {
                ccg_word.tag
            };

            match effective_tag {
                Wordclass::NNP | Wordclass::NN  => λVar!(ccg_word.text.clone()),
                Wordclass::VBZ => generate_predicate(ccg_word.text.clone(), category),
                _ => panic!("wordclass variant not implemented"),
            }
        }

    } else {
        panic!("expected word and tag on terminal node: {}", node);
    }
}

fn generate_predicate(identifier: String, category: Box<LambdaEntity>) -> Box<LambdaEntity> {
    let num_arguments = count_predicate_arguments(category.clone());

    // todo: i have no idea why this works
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

pub fn ccg_to_lambda(root: &CCGNode) -> Box<LambdaEntity> {
    ccg_to_lambda_recursive(root.clone(), root)
}

pub fn ccg_to_lambda_recursive(current_node: CCGNode, root: &CCGNode) -> Box<LambdaEntity> {
    use LambdaEntity::*;

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
            // Handle conjunction as before
            if let Some(children) = &current_node.children {
                if children.len() == 2 {
                    match (children[0].clone().node_type, children[1].clone().node_type) {
                        (CCGType::Conjunction, rhs) => {
                            λAbs!(λVar!(String::from("x1")), λConj!(λVar!(String::from("x1")), ccg_to_lambda_recursive(*children[1].clone(), root)))
                        }
                        (lhs, CCGType::Conjunction) => {
                            λAbs!(λVar!(String::from("x1")), λConj!(ccg_to_lambda_recursive(*children[0].clone(), root), λVar!(String::from("x1"))))
                        }
                        _ => panic!("Expecting CONJ type as child of Conjunction rule")
                    }
                } else { panic!("Expected 2 children in conjunction rule") }
            } else { panic!("Expected conjunction rule to have children") }
        }

        _ => panic!("Not implemented yet!")
    }
}
