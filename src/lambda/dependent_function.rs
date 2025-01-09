use std::cmp::PartialEq;
use crate::lambda::types::{Expandable, LambdaEntity, Substitutable};
use crate::lambda::conjunction::Conjunction;
use crate::{λConj, λPred, λDepFun};
use std::fmt;
use std::fmt::Formatter;
use crate::lambda::reducible::Reducible;

/// Structure to define Π(x) (expr)
#[derive(Clone, Debug, PartialEq)]
pub struct DependentFunction {
    pub bound_var: Box<LambdaEntity>,
    pub expr: Box<LambdaEntity>,
}


/// Implementation of Partial Equality for DependentFunction, used in substitution.
impl PartialEq<LambdaEntity> for DependentFunction {
    fn eq(&self, other: &LambdaEntity) -> bool {
        if let LambdaEntity::DepFun(fun) = other {
            self.bound_var == fun.bound_var && self.expr == fun.expr
        } else {
            false
        }
    }
}


/// Implementation of substitution for DependentFunction.
impl Substitutable for DependentFunction {
    fn substitute(&self, source: &LambdaEntity, target: &LambdaEntity) -> Box<LambdaEntity> {
        // Am I being substituted? If so, replace me!
        let self_as_entity = LambdaEntity::DepFun(self.clone());
        if source == &self_as_entity {
            return Box::new(target.clone());
        }

        λDepFun!(self.bound_var.substitute(source, target), self.expr.substitute(source, target))
    }
}


impl Reducible for DependentFunction {
    fn beta_reduce(&self) -> LambdaEntity {
        *λDepFun!(Box::from(self.bound_var.beta_reduce()), Box::from(self.expr.beta_reduce()))
    }
}


/// Implementation of Pretty Prints for DependentFunction
impl fmt::Display for DependentFunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "Π({}) ({})", self.bound_var, self.expr)
        }
    }



/// Implements the `Expandable` trait for `DependentFunction`, enabling recursive expansion logic.
impl Expandable for DependentFunction {
    fn expand(&self) -> Box<LambdaEntity> {
            todo!()
        }
    }
