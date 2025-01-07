use crate::lambda::types::*;


// Implement the Substitutable trait for LambdaEntity
impl Substitutable for LambdaEntity {
    /// Function to handle the recursive case of a normal-order (leftmost outermost reduction) reduction.
    fn substitute(&self, source: &LambdaEntity, target: &LambdaEntity) -> LambdaEntity {
        use LambdaEntity::*;
        match self {

            // If the node is an `Application`, we make a recursive substitution call on the bound variable
            // and the body (both sides of the dot).
            Application(left, right) => {
                let left_substituted = left.substitute(source, target);
                let right_substituted = right.substitute(source, target);
                Application(Box::new(left_substituted), Box::new(right_substituted))
            }

            // If the variable in the abstraction matches `source`, return the abstraction unchanged.
            // Otherwise, perform substitution in the body of the abstraction.
            Abstraction(variable, subexpr) => {
                if *variable == Box::from(source.clone()) {
                    Abstraction(variable.clone(), subexpr.clone())
                } else {
                    let subexpr_substituted = subexpr.substitute(source, target);
                    Abstraction(variable.clone(), Box::new(subexpr_substituted))
                }
            }

            // This is the base case! Substitute the variable if it matches `source` otherwise, return
            // as is. This might be worth changing later to subsitute subexpressions i.e. 'x + 1' -> '5 + 1'
            Variable(variable) => {
                if self == source {
                    target.clone()
                } else {
                    Variable(variable.clone())
                }
            }
        }
    }
}


/// Function to reduce a lambda expression using a normal-order reduction strategy, i.e.,
/// leftmost, outermost reduction. This now uses the `substitute` method from the `Substitutable` trait.
pub fn reduce(expression: &LambdaEntity) -> LambdaEntity {
    match expression {
        LambdaEntity::Application(expr, term) => {
            // Attempt to reduce the function part first
            let reduced_expr = reduce(expr);

            match reduced_expr {
                LambdaEntity::Abstraction(bound_var, body) => {
                    // Perform substitution: replace bound_var with term in body
                    // Extract the variable name from bound_var
                    let var_name = bound_var();
                    let substituted_body = (*body).substitute(var_name, term.as_ref());
                    // Continue reducing the substituted body
                    reduce(&substituted_body)
                }
                _ => {
                    // If the function part is not an abstraction, attempt to reduce the argument
                    let reduced_term = reduce(term);
                    LambdaEntity::Application(Box::new(reduced_expr), Box::new(reduced_term))
                }
            }
        }
        // If the expression is an abstraction, attempt to reduce its body
        LambdaEntity::Abstraction(var, body) => {
            let reduced_body = reduce(body);
            LambdaEntity::Abstraction(var.clone(), Box::new(reduced_body))
        }
        // Variables cannot be reduced further
        LambdaEntity::Variable(_) => expression.clone(),
    }
}