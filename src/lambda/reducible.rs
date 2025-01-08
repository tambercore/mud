use crate::lambda::abstraction::Abstraction;
use crate::lambda::types::*;
use crate::{λAbs, λApp};


/// Trait defining a function to reduce the lambda entity using a normal-order reduction strategy
pub trait Reducible {
    fn reduce(&self) -> LambdaEntity;
}


/// Implementation of such trait.
impl Reducible for LambdaEntity {
    fn reduce(&self) -> LambdaEntity {
        match self {
            // Handle Application: (lhs @ rhs)
            LambdaEntity::App(application) => {
                // Attempt to reduce the function part (lhs) first
                let reduced_lhs = application.lhs.reduce();

                match reduced_lhs {
                    // If the reduced lhs is an Abstraction, perform substitution
                    LambdaEntity::Abs(abstraction) => {
                        // Perform substitution: replace the bound variable with the argument (rhs) in the body
                        let substituted_body = abstraction
                            .body
                            .substitute(&abstraction.bound_var, &application.rhs);

                        // Continue reducing the substituted body
                        substituted_body.reduce()
                    }
                    // If the function part is not an Abstraction, attempt to reduce the argument (rhs)
                    _ => {
                        let reduced_rhs = application.rhs.reduce();

                        // Return the application with the reduced lhs and rhs
                        *λApp!(reduced_lhs, reduced_rhs)
                    }
                }
            }
            // Handle Abstraction: (λvar. body)
            LambdaEntity::Abs(abstraction) => {
                // Attempt to reduce the body of the abstraction
                let reduced_body = abstraction.body.reduce();

                // If the body is reduced, return the new Abstraction
                if reduced_body != *abstraction.body {
                    *λAbs!(abstraction.bound_var.clone(), Box::new(reduced_body))
                } else {
                    // If the body cannot be reduced further, return the original abstraction
                    self.clone()
                }
            }
            // Handle Variable: cannot be reduced further
            LambdaEntity::Var(_) => self.clone(),
        }
    }
}