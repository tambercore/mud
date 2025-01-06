use crate::ccg::category::CCGType;
use crate::ccg::node::CCGNode;
use crate::lambda::types::*;
use crate::ccg::type_parser::parse_category;

fn build_functor_type(_type: CCGType) -> LambdaEntity {
    match _type {
        CCGType::ForwardsFunctor(left, right) => {
            LambdaEntity::Abstraction(Box::from(build_functor_type(*right)), Box::from(build_functor_type(*left)))
        }
        CCGType::BackwardsFunctor(left, right) => {
            LambdaEntity::Abstraction(Box::from(build_functor_type(*right)), Box::from(build_functor_type(*left)))
        }
        _ => {
            LambdaEntity::Variable(_type.to_string())
        }
    }
}

#[test]
fn test_build_functor_type() {
    let mut ccg_type = parse_category("n/np").unwrap().1;
    println!("{} \n {}", ccg_type.clone(), build_functor_type(ccg_type.clone()));

    let mut ccg_type = parse_category("((s\\np)/(s\\np))").unwrap().1;
    println!("{} \n {}", ccg_type.clone(), build_functor_type(ccg_type.clone()));

    let mut ccg_type = parse_category("s/n").unwrap().1;
    println!("{} \n {}", ccg_type.clone(), build_functor_type(ccg_type.clone()));
}

fn ccg_to_lambda (root: CCGNode) -> LambdaEntity {
    panic!("");
    //match root.node_type {
    //    CCGType::ForwardsFunctor(_, _) | CCGType::BackwardsFunctor(_, _) => { build_functor_type(root) }
    //    _ => LambdaEntity::Variable(root.node_type.to_string()),
    //}
}

// (S -> NP) -> (S -> NP)
// \lambda (\lambda S . NP) . (\lambda S . NP)

// S -> NP
// \lambda x:S . y:NP

// \lambda x:S . y