use std::cmp::PartialEq;
use crate::lambda::types::{LambdaEntity, Substitutable};
use crate::λVar;
use std::fmt;
use std::fmt::Formatter;

/// Structure to define λ-Variables (x)
#[derive(Clone, Debug, PartialEq)]
pub struct Variable {
    pub name: String,
}


/// Implementation of λ-substitution for λ-Variables.
impl Substitutable for Variable {
    fn substitute(&self, source: &LambdaEntity, target: &LambdaEntity) -> Box<LambdaEntity> {
        if self == source { Box::from(target.clone()) }
        else { λVar!(self.name.clone()) }
    }
}


/// Implementation of Partial Equality for λ-Variables. Used in substitution.
impl PartialEq<LambdaEntity> for Variable {
    fn eq(&self, other: &LambdaEntity) -> bool {
        match other {
            LambdaEntity::Var(other_name) => *self.name == *other_name.name,
            _ => false
        }
    }
}


/// Implementing Pretty Print
impl fmt::Display for Variable {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}