#![allow(non_snake_case)]

use crate::brill::wordclass::Wordclass;
use crate::ccg::category::CCGType;
use crate::ccg::node::CCGNode;
use crate::ccg::rule::CCGRule;
use crate::lambda::types::*;
use crate::lambda::lambda_element::LambdaElement;

macro_rules! λVar {
    ($type_expr:expr) => {
        Box::from(Variable(Box::from($type_expr)))
    };
}

macro_rules! λAbs {
    ($left:expr, $right:expr) => {
        Box::from(Abstraction($left, $right))
    };
}

macro_rules! λApp {
    ($left:expr, $right:expr) => {
        Box::from(Application($left, $right))
    };
}


fn generate_lexical_category(_type: CCGType, _node: &CCGNode) -> Box<LambdaEntity> {
    use LambdaEntity::*;
    use LambdaElement::*;

    match _type {
        CCGType::ForwardsFunctor(left, right) | CCGType::BackwardsFunctor(left, right) => {
            let left = generate_lexical_category(*left, _node);
            let right = generate_lexical_category(*right, _node);
            let lexical_category = λAbs!(right.clone(), left.clone());

            λVar!(generate_lexical_element(_node, lexical_category))

            /*Box::from(Abstraction(
                right.clone(),
                λVar!(left.clone()),
            ))*/
        }
        _ => {
            λVar!(generate_lexical_element(_node, λVar!(Term(_type.to_string()))))
        }
    }
}

fn generate_lexical_element(node: &CCGNode, category: Box<LambdaEntity>) -> LambdaElement {
    use LambdaEntity::*;
    use LambdaElement::*;

    if let Some(ccg_word) = &node.word {
        match ccg_word.tag {
            Wordclass::NNP => Term(ccg_word.text.clone()),
            Wordclass::VBZ => {Predicate(ccg_word.text.clone(), generate_predicate_arguments(category))},
            _ => panic!("wordclass variant not implemented"),
        }
    } else {
        panic!("expected word and tag on terminal node");
    }
}


fn generate_predicate_arguments(category: Box<LambdaEntity>) -> Vec<LambdaElement> {
    // todo: count arguments
    use LambdaEntity::*;
    use LambdaElement::*;

    vec![Term("x2".to_string()), Term("x1".to_string())]
}

fn unpack_children(maybe_nodes: Option<Vec<Box<CCGNode>>>) -> (CCGNode, CCGNode) {
    let nodes_vec = maybe_nodes.expect("Expected a vector of nodes, found None.");
    let first = nodes_vec.get(0).expect("Expected at least one node, found none.");
    let second = nodes_vec.get(1).expect("Expected at least two nodes, found only one.");
    ( (**first).clone(), (**second).clone() )
}

pub fn ccg_to_lambda (root: CCGNode) -> Box<LambdaEntity> {
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
                    generate_lexical_category(children[0].node_type.clone(), &children[0])
                } else {
                    panic!("expected one child (unary rule)");
                }
            } else { panic!("expected node to have children") }
        }

        _ => panic!("not implemented yet")
    }
}