use crate::lambda::abstraction::Abstraction;
use crate::lambda::application::Application;
use crate::lambda::predicate::Predicate;
use crate::lambda::dependent_function::DependentFunction;
use crate::lambda::conjunction::Conjunction;
use crate::lambda::types::*;
use crate::{λAbs, λApp, λPred, λConj, λDepFun};


/// Trait defining a function to reduce the lambda entity using a normal-order reduction strategy
pub trait Reducible {
    fn beta_reduce(&self) -> LambdaEntity;
}


/// Implementation of such trait.
impl Reducible for LambdaEntity {
    fn beta_reduce(&self) -> LambdaEntity {
        match self {
            // Handle Application: (lhs @ rhs)
            LambdaEntity::App(application) => {
                // Attempt to reduce the function part (lhs) first
                let reduced_lhs = application.lhs.beta_reduce();

                match reduced_lhs {
                    // If the reduced lhs is an Abstraction, perform substitution
                    LambdaEntity::Abs(abstraction) => {
                        // Perform substitution: replace the bound variable with the argument (rhs) in the body
                        let substituted_body = abstraction
                            .body
                            .substitute(&abstraction.bound_var, &application.rhs);

                        // Continue reducing the substituted body
                        substituted_body.beta_reduce()
                    }
                    // If the function part is not an Abstraction, attempt to reduce the argument (rhs)
                    _ => {
                        let reduced_rhs = application.rhs.beta_reduce();

                        // Return the application with the reduced lhs and rhs
                        *λApp!(Box::from(reduced_lhs), Box::from(reduced_rhs))
                    }
                }
            }
            // Handle Abstraction: (λvar. body)
            LambdaEntity::Abs(abstraction) => {
                // Attempt to reduce the body of the abstraction
                let reduced_body = abstraction.body.beta_reduce();

                // If the body is reduced, return the new Abstraction
                if reduced_body != *abstraction.body {
                    *λAbs!(abstraction.bound_var.clone(), Box::new(reduced_body))
                } else {
                    // If the body cannot be reduced further, return the original abstraction
                    self.clone()
                }
            }

            LambdaEntity::Pred(predicate) => {
                // Reduce all arguments of the predicate
                let reduced_args: Vec<Box<LambdaEntity>> = predicate.args
                    .iter()
                    .map(|arg| Box::new(arg.beta_reduce()))
                    .collect();

                *λPred!(predicate.iden.clone(), reduced_args)

            }

            LambdaEntity::Conj(conjunction) => {
                *λConj!(Box::from(conjunction.lhs.beta_reduce()), Box::from(conjunction.rhs.beta_reduce()))
            }

            LambdaEntity::DepFun(dep) => {
                *λDepFun!(Box::from(dep.bound_var.clone()), Box::from(dep.expr.beta_reduce()))
            }

            // Handle Non Computational Cases (i.e. vars and predicates)
            _ => self.clone(),
        }
    }
}