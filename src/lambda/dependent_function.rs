use std::cmp::PartialEq;
use crate::lambda::types::{Expandable, LambdaEntity, Substitutable};
use crate::lambda::conjunction::Conjunction;
use crate::{λConj, λPred};
use std::fmt;
use std::fmt::Formatter;
use crate::lambda::reducible::Reducible;

/// Structure to define Π(x) (expr)
#[derive(Clone, Debug, PartialEq)]
pub struct DependentFunction {
    pub bound_var: Box<LambdaEntity>, // TODO: should this be a String (Man) or a Term?
    pub expr: Box<LambdaEntity>,
}


/// Implementation of Partial Equality for DependentFunction, used in substitution.
impl PartialEq<LambdaEntity> for DependentFunction {
    fn eq(&self, other: &LambdaEntity) -> bool {
        todo!()
    }
}


/// Implementation of substitution for DependentFunction.
impl Substitutable for DependentFunction {
    fn substitute(&self, source: &LambdaEntity, target: &LambdaEntity) -> Box<LambdaEntity> {
        todo!()
    }
}


impl Reducible for DependentFunction {
    fn beta_reduce(&self) -> LambdaEntity {
        todo!()
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
