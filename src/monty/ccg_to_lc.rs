use std::thread::current;
use crate::ccg::category::CCGType;
use crate::ccg::node::{unpack_children, CCGNode};
use crate::ccg::rule::CCGRule::{BackwardApplication, ForwardApplication};
use crate::lambda::types::LambdaEntity;
use crate::lambda::variable::Variable;
use crate::lambda::conjunction::Conjunction;
use crate::lambda::abstraction::Abstraction;
use crate::monty::lambda_generation::ccg_to_quantifier;
use crate::{λAbs, λApp, λConj, λVar};
use crate::ccg::rule::CCGRule;
use crate::lambda::application::Application;
use crate::monty::handle_lexical::lexical_to_lambda;



pub fn ccg_to_lambda(root: &mut CCGNode) -> Box<LambdaEntity> {
    ccg_to_lambda_recursive(root.clone(), root)
}



pub fn ccg_to_lambda_recursive(current_node: CCGNode, root: &CCGNode) -> Box<LambdaEntity> {
    use LambdaEntity::*;

    match current_node.rule {

        /* Generates λ-Applications */
        ForwardApplication | BackwardApplication => {

            /* Transform to make ForwardApplication = BackwardApplication */
            let (first, second) = unpack_children(current_node.children);
            let (left, right) = if let BackwardApplication = current_node.rule { (second, first) } else { (first, second) };

            /* Construct Application */
            λApp!(
                ccg_to_lambda_recursive(left, root),
                ccg_to_lambda_recursive(right, root)
            )
        }

        /* Generates λ-Terms */
        CCGRule::Lexical => {
            lexical_to_lambda(current_node)
        }

        /* Skips to Child */
        CCGRule::Unary => {
            let children = current_node.children.as_ref().expect("Expected node to have children.");
            if children.len() != 1 { panic!("Expected one child (unary rule)."); }
            ccg_to_lambda_recursive(*children[0].clone(), root)
        }

        /* Handles Conjunctions as a Product Type */
        CCGRule::Conjunction => {
            let (_, right) = unpack_children(current_node.children);
            let conj_body = λConj!(λVar!("x꜀".to_string()), ccg_to_lambda_recursive(right, root));
            return λAbs!(λVar!("x꜀".to_string()), conj_body);
        }

        _ => { panic!("Aah!") }
    }
}