use crate::ccg::category::CCGType;
use crate::ccg::node::CCGNode;
use crate::lambda::types::*;

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
    let ccg_type = CCGType::ForwardsFunctor(Box::from(CCGType::NounPhrase), Box::from(CCGType::Noun));
    println!("{} \n {}", ccg_type.clone(), build_functor_type(ccg_type.clone()));
    assert_eq!(build_functor_type(ccg_type.clone()), LambdaEntity::Abstraction(Box::from(LambdaEntity::Variable(String::from("N"))), Box::from(LambdaEntity::Variable(String::from("NP")))));
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