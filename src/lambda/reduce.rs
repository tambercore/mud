use crate::lambda::types::*;
use crate::lambda::types::LambdaEntity::{Application, Abstraction, Variable};


/// Function to handle the recursive case of a normal-order (leftmost outermost reduction) reduction.
fn _substitute(expression: &LambdaEntity, source: &str, target: &LambdaEntity) -> LambdaEntity {
    match expression {

        // If the node is an `Application`, we make a recursive substitution call on the bound variable
        // and the body (both sides of the dot).
        Application(left, right) => {
            let left_substituted = _substitute(left, source, target);
            let right_substituted = _substitute(right, source, target);
            Application(Box::new(left_substituted), Box::new(right_substituted))
        }

        // If the variable in the abstraction matches `source`, return the abstraction unchanged.
        // Otherwise, perform substitution in the body of the abstraction.
        Abstraction(variable, subexpr) => {
            if *variable == source {
                Abstraction(variable.clone(), subexpr.clone())
            } else {
                let subexpr_substituted = _substitute(subexpr, source, target);
                Abstraction(variable.clone(), Box::new(subexpr_substituted))
            }
        }

        // This is the base case! Substitute the variable if it matches `source` otherwise, return
        // as is. This might be worth changing later to subsitute subexpressions i.e. 'x + 1' -> '5 + 1'
        Variable(variable) => {
            if variable == source {
                target.clone()
            } else {
                Variable(variable.clone())
            }
        }
    }
}



/// Function to reduce a lambda expression using a normal-order reduction strategy, i.e.
/// leftmost, outermost reduction. This uses the recursive `substitute` func defined above.
pub fn reduce(expression: &LambdaEntity) -> LambdaEntity {
    match expression {
        Application(expr, term) => {
            // Reduce the function (expr) and argument (term) before applying substitution.
            let reduced_expr = reduce(expr);
            let reduced_term = reduce(term);

            // Pattern match on `reduced_expr` directly, without dereferencing.
            if let Abstraction(var, body) = reduced_expr {
                let substituted_body = _substitute(&body, &var, &reduced_term);
                reduce(&substituted_body)
            } else {
                // If `reduced_expr` is not an abstraction, return the application as is.
                Application(Box::new(reduced_expr), Box::new(reduced_term))
            }
        }
        // If the expression is not an application, return it as is.
        _ => expression.clone(),
    }
}
