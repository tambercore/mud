#![allow(non_snake_case)]

use crate::ccg::category::CCGType;
use crate::ccg::node::CCGNode;
use crate::ccg::rule::CCGRule;
use crate::lambda::types::*;
use crate::ccg::type_parser::parse_category;
use crate::lambda::lambda_element::LambdaElement;

macro_rules! λVar {
    ($type_expr:expr) => {
        Box::from(LambdaEntity::Variable(Box::from(LambdaElement::Term($type_expr.to_string()))))
    };
}

macro_rules! λAbs {
    ($left:expr, $right:expr) => {
        Box::from(Abstraction($right, $left))
    };
}

macro_rules! λApp {
    ($left:expr, $right:expr) => {
        Box::from(Application($right, $left))
    };
}


fn generate_lexical_category(_type: CCGType, _node: &CCGNode) -> Box<LambdaEntity> {
    use LambdaEntity::*;
    use LambdaElement::*;
    match _type {
        CCGType::ForwardsFunctor(left, right) => {
            λAbs!(generate_lexical_category(*left, _node), generate_lexical_category(*right, _node))
        }
        CCGType::BackwardsFunctor(left, right) => {
            λAbs!(generate_lexical_category(*left, _node), generate_lexical_category(*right, _node))
        }
        _ => {
            λVar!(_type)
        }
    }
}

fn unpack_children(maybe_nodes: Option<Vec<Box<CCGNode>>>) -> (CCGNode, CCGNode) {
    let nodes_vec = maybe_nodes.expect("Expected a vector of nodes, found None.");
    let first = nodes_vec.get(0).expect("Expected at least one node, found none.");
    let second = nodes_vec.get(1).expect("Expected at least two nodes, found only one.");
    ( (**first).clone(), (**second).clone() )
}

pub fn ccg_to_lambda(root: CCGNode) -> Box<LambdaEntity> {
    use LambdaEntity::*;
    match root.rule {
        // Base case: terminal nodes
        CCGRule::Lexical => {
            let _type = generate_lexical_category(root.node_type.clone(), &root);
            _type
        },

        // Recursive case
        CCGRule::BackwardApplication => {
            let (left, right) = unpack_children(root.children);
            λApp!(ccg_to_lambda(right), ccg_to_lambda(left))
        },
        CCGRule::ForwardApplication => {
            let (left, right) = unpack_children(root.children);
            λApp!(ccg_to_lambda(left), ccg_to_lambda(right))
        }
        CCGRule::Unary => generate_lexical_category(root.node_type.clone(), &root),

        _ => panic!("not implemented yet"),
    }
}
