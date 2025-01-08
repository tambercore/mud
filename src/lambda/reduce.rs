use crate::lambda::abstraction::Abstraction;
use crate::lambda::types::*;


/// Function to reduce a lambda expression using a normal-order reduction strategy, i.e.,
/// leftmost, outermost reduction. This now uses the `substitute` method from the `Substitutable` trait.
pub fn reduce(expression: &LambdaEntity) -> LambdaEntity {
    match expression {
        // Handle Application: (lhs @ rhs)
        LambdaEntity::App(application) => {
            // Attempt to reduce the function part (lhs) first
            let reduced_lhs = reduce(&application.lhs);

            match reduced_lhs {
                // If the reduced lhs is an Abstraction, perform substitution
                LambdaEntity::Abs(abstraction) => {
                    // Perform substitution: replace the bound variable with the argument (rhs) in the body
                    let substituted_body = abstraction.body.substitute(&abstraction.bound_var, &application.rhs);

                    // Continue reducing the substituted body
                    reduce(&substituted_body)
                }
                // If the function part is not an Abstraction, attempt to reduce the argument (rhs)
                _ => {
                    let reduced_rhs = reduce(&application.rhs);

                    // If both lhs and rhs are fully reduced, return the application
                    if reduced_rhs == *application.rhs {
                        LambdaEntity::App(Box::new(reduced_lhs), Box::new(reduced_rhs))
                    } else {
                        // Otherwise, continue reducing
                        LambdaEntity::App(Box::new(reduced_lhs), Box::new(reduced_rhs))
                    }
                }
            }
        }
        // Handle Abstraction: (λvar. body)
        LambdaEntity::Abs(abstraction) => {
            // Attempt to reduce the body of the abstraction
            let reduced_body = reduce(&abstraction.body);

            // If the body is reduced, return the new Abstraction
            if reduced_body != *abstraction.body {
                LambdaEntity::Abs(Box::new(Abstraction {
                    bound_var: abstraction.bound_var.clone(),
                    body: Box::new(reduced_body),
                }))
            } else {
                // If the body cannot be reduced further, return the original abstraction
                expression.clone()
            }
        }
        // Handle Variable: cannot be reduced further
        LambdaEntity::Var(_) => expression.clone(),
    }
}