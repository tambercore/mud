use crate::lambda::abstraction::Abstraction;
use crate::lambda::application::Application;
use crate::lambda::predicate::Predicate;
use crate::lambda::conjunction::Conjunction;
use crate::lambda::variable::Variable;
use crate::lambda::types::*;
use crate::{λAbs, λApp, λPred, λConj, λVar};
use crate::lambda::reducible::Reducible;

/// Trait defining a function to reduce the lambda entity using a normal-order reduction strategy
pub trait Eliminator {
    fn eliminate_leftovers(&self) -> LambdaEntity;
}


/// Implementation of such trait.
impl Eliminator for LambdaEntity {
    fn eliminate_leftovers(&self) -> LambdaEntity {
        match self {
            LambdaEntity::Abs(abstraction) => {
                match (*abstraction.clone().bound_var, *abstraction.clone().body) {
                    (LambdaEntity::Var(v), LambdaEntity::Pred(inner_pred)) => {
                        if inner_pred.args.len() == 1 {
                            if inner_pred.args[0] == Box::from(LambdaEntity::Var(v)) {
                                return *λVar!(inner_pred.iden)
                            }
                        }

                    }
                    _ => {}
                }
                return self.clone()
            }

            /* Recur into predicate arguments */
            LambdaEntity::Pred(predicate) => {

                /*
                 * This matches the general pattern for non `is` relations, usually applying
                 * to modals, such as necessity and possibility.
                 */
                if (predicate.iden != "is" && predicate.args.len() == 2) {
                    match *predicate.args[1].clone() {
                        LambdaEntity::Abs(func) => {
                             return *λPred!(predicate.iden.clone(), vec![
                                 Box::from(*λApp!(
                                     predicate.args[1].clone(),
                                     predicate.args[0].clone()
                                 ))]).beta_reduce().eliminate_leftovers().expand();
                        }
                        _ => (),
                    }
                }

                let reduced_args: Vec<Box<LambdaEntity>> = predicate.args
                    .iter()
                    .map(|arg| Box::new(arg.eliminate_leftovers()))
                    .collect();
                *λPred!(predicate.iden.clone(), reduced_args)
            }

            /* Recur into conjunction sides */
            LambdaEntity::Conj(conjunction) => {
                *λConj!(Box::from(conjunction.lhs.eliminate_leftovers()), Box::from(conjunction.rhs.eliminate_leftovers()))
            }

            _ => { self.clone() }

            /*
            LambdaEntity::App(_) => {}
            LambdaEntity::Var(_) => {}
            LambdaEntity::CaseH(_) => {}
             */
        }
    }
}