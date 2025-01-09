#![allow(non_snake_case)]

use crate::brill::wordclass::Wordclass;
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
use crate::brill::brill_tagger::get_possible_tags;
use crate::brill::init_tagger::{initialize_tagger, WordclassMap};

fn generate_lexical_category(_type: CCGType, _node: &CCGNode) -> Box<LambdaEntity> {
    

    match _type {
        CCGType::ForwardsFunctor(left, right) | CCGType::BackwardsFunctor(left, right) => {
            let lexical_category = λAbs!(generate_lexical_category(*right, _node), generate_lexical_category(*left, _node));
            generate_lexical_element(_node, lexical_category)
        }
        _ => generate_lexical_element(_node, λVar!(_type.to_string()))
    }
}


fn generate_lexical_element(node: &CCGNode, category: Box<LambdaEntity>) -> Box<LambdaEntity> {
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
            Wordclass::NNP => λVar!(ccg_word.text.clone()),
            Wordclass::VBZ => generate_predicate(ccg_word.text.clone(), category),
            _ => panic!("wordclass variant not implemented"),
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


pub fn ccg_to_lambda (root: CCGNode) -> Box<LambdaEntity> {
    //use CCGRule::*;
    use LambdaEntity::*;

    
    match root.rule {
        // Base case: terminal nodes
        CCGRule::Lexical => generate_lexical_category(root.node_type.clone(), &root),

        // Recursive case
        CCGRule::BackwardApplication => {
            let (left, right) = unpack_children(root.children);
            λApp!(ccg_to_lambda(right), ccg_to_lambda(left))

        },
        CCGRule::ForwardApplication => {
            let (left, right) = unpack_children(root.children);
            λApp!(ccg_to_lambda(left), ccg_to_lambda(right))
        }
        CCGRule::Unary => {
            if let Some(children) = &root.children {
                if children.len() == 1 {
                    ccg_to_lambda(*children[0].clone())
                } else {
                    panic!("Expected one child (unary rule).");
                }
            } else { panic!("Expected node to have children.") }
        }
        CCGRule::Conjunction => {
            // gouda \ (and cheddar)
            // if the "and" first combines with the left argument (gouda and) / cheddar.
            // im not sure if lambeq does this
            // todo fix indentation

            // if the type of the first argument is CONJ, then it is of the form "and --right--"
            // lambda x . (x and ccg_to_lambda(arg2))
            if let Some(children) = &root.children {
                if children.len() == 2 {
                    match (children[0].clone().node_type, children[1].clone().node_type) {
                        (CCGType::Conjunction, rhs) => {λAbs!(λVar!(String::from("x1")), λConj!(λVar!(String::from("x1")), ccg_to_lambda(*children[1].clone())))}
                        (lhs, CCGType::Conjunction) => {λAbs!(λVar!(String::from("x1")), λConj!(ccg_to_lambda(*children[0].clone()), λVar!(String::from("x1"))))}
                        _ => panic!("Expecting CONJ type as child of Conjunction rule")

                    }
                } else {panic!("Expected 2 children in conjunction rule")}
            } else {panic!("Expected conjunction rule to have children")}

            // if the type of the second argument is CONJ, then it is of the form "--left-- and"
            // lambda x . (ccg_to_lambda(arg1) and x)



        }

        _ => panic!("Not implemented yet!")
    }
}