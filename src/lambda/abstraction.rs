use crate::lambda::types::{LambdaEntity, Substitutable};
use crate::λAbs;


/// Structure to define λ-Abstractions (λx. e)
pub struct Abstraction {
    pub bound_var: Box<LambdaEntity>,
    pub body: Box<LambdaEntity>,
}


/// Implementation of λ-substitution for λ-Abstractions.
impl Substitutable for Abstraction {
    fn substitute(&self, source: &LambdaEntity, target: &LambdaEntity) -> Box<LambdaEntity> {
        let bound_variable = &self.bound_var;
        let subexpr = &self.body;

        // If we're substituting the bound variable ignore.
        // TODO: We should fix this at some point, using De Bruijin Indicies, renaming the BV.
        if *&self.bound_var == Box::from(source.clone()) {
            λAbs!(bound_variable.clone(), subexpr.clone())

        // Otherwise, substitute in the sub expression.
        } else {
            let subexpr_substituted: Box<LambdaEntity> = subexpr.substitute(source, target);
            λAbs!(bound_variable.clone(), subexpr_substituted)
        }
    }
}


/// Implementation of Partial Equality, used for substitution.
impl PartialEq for Abstraction {
    fn eq(&self, other: &Self) -> bool {
        self.bound_var == other.bound_var && self.body == other.body
    }
}