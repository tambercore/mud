use crate::lambda::types::*;
use crate::lambda::types::LambdaEntity::{Application, Abstraction, Variable};


fn _substitute(expression: &LambdaEntity, source: &str, target: &LambdaEntity) -> LambdaEntity {
    match expression {
        Application(left, right) => {
            let left_substituted = _substitute(left, source, target);
            let right_substituted = _substitute(right, source, target);
            Application(Box::new(left_substituted), Box::new(right_substituted))
        }
        Abstraction(variable, subexpr) => {
            if *variable == source {
                // If the variable in the abstraction matches `source`, return the abstraction unchanged.
                Abstraction(variable.clone(), subexpr.clone())
            } else {
                // Otherwise, perform substitution in the body of the abstraction.
                let subexpr_substituted = _substitute(subexpr, source, target);
                Abstraction(variable.clone(), Box::new(subexpr_substituted))
            }
        }
        Variable(variable) => {
            // Substitute the variable if it matches `source`; otherwise, return as is.
            if variable == source {
                target.clone()
            } else {
                Variable(variable.clone())
            }
        }
    }
}

fn substitute(expression: &LambdaEntity, target: &LambdaEntity) -> LambdaEntity {
    match expression {
        Abstraction(variable, subexpr) => {
            // Apply substitution to the body of the abstraction.
            _substitute(subexpr, variable, target)
        }
        Application(_, _) | Variable(_) => expression.clone(),
    }
}

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
