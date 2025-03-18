use std::thread::current;
use crate::ccg::category::CCGType;
use crate::ccg::node::{unpack_children, CCGNode};
use crate::ccg::rule::CCGRule::{BackwardApplication, ForwardApplication};
use crate::lambda::types::LambdaEntity;
use crate::lambda::variable::Variable;
use crate::lambda::conjunction::Conjunction;
use crate::lambda::abstraction::Abstraction;
use crate::{λAbs, λApp, λConj, λVar};
use crate::ccg::rule::CCGRule;
use crate::lambda::application::Application;
use crate::monty::handle_lexical::lexical_to_lambda;

/// Function to convert a CCG parse tree into a λ-term.
pub fn ccg_to_lambda(root: &mut CCGNode) -> Box<LambdaEntity> {
    ccg_to_lambda_recursive(root.clone(), root)
}



/// Function to recursively transform CCG nodes into λ-terms.
pub fn ccg_to_lambda_recursive(current_node: CCGNode, root: &CCGNode) -> Box<LambdaEntity> {
    use LambdaEntity::*;

    match current_node.rule {
        /* Generates λ-Applications */
        ForwardApplication | BackwardApplication => {
            /* Transform to make ForwardApplication = BackwardApplication */
            let (first, second) = unpack_children(current_node.children);
            let (left, right) = if let BackwardApplication = current_node.rule {
                (second, first)
            } else {
                (first, second)
            };

            /* Construct Application */
            λApp!(
                ccg_to_lambda_recursive(left, root),
                ccg_to_lambda_recursive(right, root)
            )
        }

        CCGRule::BackwardCrossedComposition => {
            let (first_child, second_child) = unpack_children(current_node.children);

            /*
             * Because it's backward, you may need to swap:
             * or not, depending on how your parse is stored!
             * The key is: "the function that expects y" gets applied to
             * "the function that returns y from z."
             */

            let left_expr = ccg_to_lambda_recursive(first_child, root);
            let right_expr = ccg_to_lambda_recursive(second_child, root);

            /* Build λz. left_expr ( right_expr z ) */
            λAbs!(
                λVar!("z".parse().unwrap()),
                λApp!(
                    left_expr,
                    λApp!(right_expr, λVar!("z".parse().unwrap()))
                )
            )
        }

        /* Generates λ-Terms */
        CCGRule::Lexical => {
            let expr = lexical_to_lambda(current_node.clone());
            // println!("{} generated {}", current_node.word.unwrap().text, expr);
            expr
        }

        /* Skips to Child */
        CCGRule::Unary => {
            let children = current_node.children.as_ref().expect("Expected node to have children.");
            if children.len() != 1 {
                panic!("Expected one child (unary rule).");
            }
            ccg_to_lambda_recursive(*children[0].clone(), root)
        }

        /* Handles Conjunctions as a Product Type */
        CCGRule::Conjunction => {
            let (_, right) = unpack_children(current_node.children);
            let conj_body = λConj!(λVar!("x꜀".to_string()), ccg_to_lambda_recursive(right, root));
            λAbs!(λVar!("x꜀".to_string()), conj_body)
        }

        _ => {
            panic!("Critical: CCGRule not implemented for Lambda Calculus conversion!")
        }
    }
}
