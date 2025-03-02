use std::cmp::PartialEq;
use crate::lambda::types::{Expandable, LambdaEntity, Substitutable};
use crate::lambda::conjunction::Conjunction;
use crate::{λConj, λPred};
use std::fmt;
use std::fmt::Formatter;
use crate::lambda::reducible::Reducible;

/// Structure to define Predicates i.e. P(x)
#[derive(Clone, Debug, PartialEq)]
pub struct Predicate {
    pub iden: String,
    pub args: Vec<Box<LambdaEntity>>
}


/// Implementation of Partial Equality for Predicates, used in substitution.
impl PartialEq<LambdaEntity> for Predicate {
    fn eq(&self, other: &LambdaEntity) -> bool {
        if let LambdaEntity::Pred(pred) = other {
            self.iden == pred.iden && self.args == pred.args
        } else {
            false
        }
    }
}


/// Implementation of substitution for Predicates.
impl Substitutable for Predicate {
    fn substitute(&self, source: &LambdaEntity, target: &LambdaEntity) -> Box<LambdaEntity> {
        // Am I being substituted? If so, replace me!
        let self_as_entity = LambdaEntity::Pred(self.clone());
        if source == &self_as_entity {
            return Box::new(target.clone());
        }

        // Otherwise, substitute within predicate arguments
        let substituted_args: Vec<Box<LambdaEntity>> = self.args
            .iter()
            .map(|arg| arg.substitute(source, target))
            .collect();

        λPred!(self.iden.clone(), substituted_args)
    }
}


impl Reducible for Predicate {

    fn beta_reduce(&self) -> LambdaEntity {
        // Reduce all arguments of the predicate
        let reduced_args: Vec<Box<LambdaEntity>> = self.args
            .iter()
            .map(|arg| Box::new(arg.beta_reduce()))
            .collect();

        *λPred!(self.iden.clone(), reduced_args)
    }
}


/// Implementation of Pretty Prints for Predicates
impl fmt::Display for Predicate {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.args.is_empty() { write!(f, "{}", self.iden) }
        else {
            let args_str = self.args.iter().map(|arg| arg.to_string()).collect::<Vec<String>>().join(", ");
            write!(f, "{}({})", self.iden, args_str)
        }
    }
}


/// Utility function to create a new predicate by modifying its arguments using a provided function.
fn modify_predicate_args<F>(predicate: &Predicate, modify: F) -> Box<LambdaEntity> where F: Fn(&LambdaEntity) -> Box<LambdaEntity>,
{
    let new_args: Vec<_> = predicate
        .args
        .iter()
        .map(|arg| modify(&**arg))
        .collect();

    Box::new(LambdaEntity::Pred(Predicate {
        iden: predicate.iden.clone(),
        args: new_args,
    }))
}


/// Implements the `Expandable` trait for `Predicate`, enabling recursive expansion logic.
impl Expandable for Predicate {
    fn expand(&self) -> Box<LambdaEntity> {
        // Create a mutable copy of the current predicate.
        let mut modified_predicate = self.clone();

        // Modify in-place
        for i in 0..modified_predicate.args.len() {
            if let LambdaEntity::Pred(inner_pred) = &*modified_predicate.args[i] {
                let expanded_pred = inner_pred.expand();
                modified_predicate.args[i] = expanded_pred;
            }
            if let LambdaEntity::Conj(conjunction) = &*modified_predicate.args[i] {
                // For conjunction, create two new predicates with lhs and rhs respectively.
                let lhs_pred = modify_predicate_args(&modified_predicate, |a| {
                    if *a == *modified_predicate.args[i] {
                        Box::new(*conjunction.lhs.clone())
                    } else {
                        Box::from(a.clone())
                    }
                });
                let rhs_pred = modify_predicate_args(&modified_predicate, |a| {
                    if *a == *modified_predicate.args[i] {
                        Box::new(*conjunction.rhs.clone())
                    } else {
                        Box::from(a.clone())
                    }
                });
                let splitted = λConj!(lhs_pred, rhs_pred);
                // Recur on the split predicate.
                return splitted.expand();
            }
        }
        // Return the modified predicate.
        Box::new(LambdaEntity::Pred(modified_predicate))
    }
}