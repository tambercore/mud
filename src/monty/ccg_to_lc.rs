use std::thread::current;
use crate::ccg::category::CCGType;
use crate::ccg::node::{unpack_children, CCGNode};
use crate::ccg::rule::CCGRule::{BackwardApplication, ForwardApplication};
use crate::lambda::types::LambdaEntity;
use crate::lambda::variable::Variable;
use crate::monty::lambda_generation::ccg_to_quantifier;
use crate::{λApp, λVar};
use crate::ccg::rule::CCGRule;
use crate::lambda::application::Application;
use crate::monty::handle_lexical::lexical_to_lambda;

pub fn ccg_to_lambda(root: &mut CCGNode) -> Box<LambdaEntity> {
    ccg_to_lambda_recursive(root.clone(), root)
}



pub fn ccg_to_lambda_recursive(current_node: CCGNode, root: &CCGNode) -> Box<LambdaEntity> {
    use LambdaEntity::*;

    // Handle quantifier nodes
    if current_node.is_quantification_node() {
        return ccg_to_quantifier(current_node.clone(), root);
    }

    match current_node.rule {

        /* Handle Applications */
        ForwardApplication => {
            let (left, right) = unpack_children(current_node.children);
            λApp!(
                ccg_to_lambda_recursive(left, root),
                ccg_to_lambda_recursive(right, root)
            )
        }

        BackwardApplication => {
            let (right, left) = unpack_children(current_node.children);
            λApp!(
                ccg_to_lambda_recursive(left, root),
                ccg_to_lambda_recursive(right, root)
            )
        }

        CCGRule::Lexical => {
            lexical_to_lambda(current_node)
        }

        CCGRule::Unary => {
            if let Some(children) = &current_node.children {
                if children.len() == 1 {
                    return ccg_to_lambda_recursive(*children[0].clone(), root);
                } else {
                    panic!("Expected one child (unary rule).");
                }
            } else {
                panic!("Expected node to have children.");
            }
        }

        _ => { panic!("Aah!") }
    }
}