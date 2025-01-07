use crate::lambda::types::{LambdaEntity, Substitutable};
use crate::λApp;


/// Structure to define λ-Variables (x)
pub struct Variable {
    pub name: String,
}


/// Implementation of λ-substitution for λ-Variables.
impl Substitutable for Variable {
    fn substitute(&self, source: &LambdaEntity, target: &LambdaEntity) -> Box<LambdaEntity> {
        if self == source { *target.clone() }
        else { λVar!(self.name.clone()) }
    }
}


/// Implementation of Partial Equality for λ-Variables. Used in substitution.
impl PartialEq for Variable {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}