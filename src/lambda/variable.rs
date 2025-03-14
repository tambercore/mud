use std::cmp::PartialEq;
use crate::lambda::types::{LambdaEntity, Substitutable};
use crate::λVar;
use std::fmt;
use std::fmt::Formatter;
use uuid::Uuid;



/// Structure to define [`Variable`]/λ-terms. These represent mathematical
/// placeholders that solely exist to be substituted during application.
#[derive(Clone, Debug, PartialEq)]
pub struct Variable {
    pub name: String,
    pub id: Option<Uuid>,
}



/// Implementation of Substitution for [`Variable`]. This is the base case
/// for all substitution in the λ-calculus, and swaps if a match occurs.
impl Substitutable for Variable {
    fn substitute(&self, source: &LambdaEntity, target: &LambdaEntity) -> Box<LambdaEntity> {
        if self == source { Box::from(target.clone()) }
        else { λVar!(self.name.clone()) }
    }
}



/// Implementation of Partial Equality for [`Variable`], states that equality
/// of [`Variable`] maps to the equality of the `name` attributes. Additionally,
/// this states that any non-variable lambda entity is inequal to any [`Variable`].
impl PartialEq<LambdaEntity> for Variable {
    fn eq(&self, other: &LambdaEntity) -> bool {
        match other {
            LambdaEntity::Var(other_name) => *self.name == *other_name.name,
            _ => false
        }
    }
}



/// Implementation of '_pretty print_' for [`Variable`] - used to generate
/// displays of wider expressions in the λ-calculus.
impl fmt::Display for Variable {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}