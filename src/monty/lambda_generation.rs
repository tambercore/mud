use crate::ccg::category::CCGType;
use crate::ccg::node::CCGNode;
use crate::ccg::rule::CCGRule;
use crate::lambda::types::*;
use crate::ccg::type_parser::parse_category;

fn generate_lexical_category(_type: CCGType) -> Box<LambdaEntity> {
    use LambdaEntity::*;
    match _type {
        CCGType::ForwardsFunctor(left, right) => {
            Box::from(Abstraction(generate_lexical_category(*right), generate_lexical_category(*left)))
        }
        CCGType::BackwardsFunctor(left, right) => {
            Box::from(Abstraction(generate_lexical_category(*right), generate_lexical_category(*left)))
        }
        _ => {
            Box::from(Variable(_type.to_string()))
        }
    }
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
        CCGRule::Lexical => generate_lexical_category(root.node_type),

        // Recursive case
        CCGRule::BackwardApplication => {
            let (left, right) = unpack_children(root.children);
            Box::from(Application(ccg_to_lambda(right), ccg_to_lambda(left)))

        },
        CCGRule::ForwardApplication => {
            let (left, right) = unpack_children(root.children);
            Box::from(Application(ccg_to_lambda(left), ccg_to_lambda(right)))
        }
        CCGRule::Unary => generate_lexical_category(root.node_type),

        _ => panic!("not implemented yet")
    }
}

#[test]
fn test_build_functor_type() {
    let mut ccg_type = parse_category("n/np").unwrap().1;
    println!("{} \n {}", ccg_type.clone(), generate_lexical_category(ccg_type.clone()));

    let mut ccg_type = parse_category("((s\\np)/(s\\np))").unwrap().1;
    println!("{} \n {}", ccg_type.clone(), generate_lexical_category(ccg_type.clone()));

    let mut ccg_type = parse_category("s/n").unwrap().1;
    println!("{} \n {}", ccg_type.clone(), generate_lexical_category(ccg_type.clone()));
}