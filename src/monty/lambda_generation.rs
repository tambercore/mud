use crate::ccg::category::CCGType;
use crate::ccg::node::CCGNode;
use crate::lambda::types::*;

fn build_functor_type(_type: CCGType) -> LambdaEntity {
    match _type {
        CCGType::ForwardsFunctor(left, right) => {
            panic!("");
            // return LambdaEntity::Abstraction(build_functor_type(*left), build_functor_type(*right));
        }
        CCGType::BackwardsFunctor(left, right) => {
            panic!("")
        }
        _ => { panic!("Expected a functor type, got other.") }
    }
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