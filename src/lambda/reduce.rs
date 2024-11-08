
use crate::lambda::types::*;
use crate::lambda::types::LambdaEntity::Variable;

fn _substitute(expression: &LambdaEntity, source: &str, target: &LambdaEntity) -> LambdaEntity {
    match expression {
        LambdaEntity::Application(left, right) => {
            // Recursively substitute the left and right parts, returning the owned value
            let left_substituted = _substitute(left, source, target);
            let right_substituted = _substitute(right, source, target);
            LambdaEntity::Application(
                Box::new(left_substituted), // No need for clone, we move the values here
                Box::new(right_substituted),
            )
        }
        LambdaEntity::Abstraction(variable, subexpr) => {
            if *variable == source {
                // If the variable matches, return the abstraction as it is
                LambdaEntity::Abstraction(variable.clone(), subexpr.clone())
            } else {
                // Otherwise, recursively substitute in the subexpression
                let subexpr_substituted = _substitute(subexpr, source, target);
                LambdaEntity::Abstraction(variable.clone(), Box::new(subexpr_substituted))
            }
        }
        LambdaEntity::Variable(variable) => {
            if *variable == source {
                // Return a new Variable with the target value
                LambdaEntity::Variable(target.to_string())
            } else {
                expression.clone() // Clone the original variable if no match
            }
        }
    }
}

fn substitute(expression: &LambdaEntity, target: &LambdaEntity) -> LambdaEntity {
    match expression {
        LambdaEntity::Abstraction(variable, subexpr) => {
            // Perform substitution for the abstraction
            _substitute(subexpr, variable, target)
        }
        LambdaEntity::Application(_, _) => expression.clone(), // Clone the application
        LambdaEntity::Variable(_) => expression.clone(), // Clone the variable
    }
}

pub fn reduce(expression: &LambdaEntity) -> LambdaEntity {
    match expression {
        LambdaEntity::Application(expr, term) => {
            return substitute(expr, term)
        }
        _ => expression.clone()
    }
}
