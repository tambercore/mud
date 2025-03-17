use crate::lambda::types::{LambdaEntity, Substitutable};
use crate::λAbs;
use std::fmt;
use std::fmt::Formatter;



/// Structure to define [`Abstraction`]/λ-Abs. These represent λ-expressions of
/// the form (λx. e) in the λ-calculus, where a bound variable is declared with
/// a body expression that may depend (contain) on that bound variable.
#[derive(Clone, Debug, PartialEq)]
pub struct Abstraction {
    pub bound_var: Box<LambdaEntity>,
    pub body: Box<LambdaEntity>,
}



/// Implementation of [`Substitutable`] for [`Abstraction`]. This captures the
/// process of substituting a source λ-term with another λ-term (target) inside
/// the body of the abstraction. Comparable to β-reduction.
impl Substitutable for Abstraction {
    fn substitute(&self, source: &LambdaEntity, target: &LambdaEntity) -> Box<LambdaEntity> {

        let bound_variable = &self.bound_var;
        let subexpr = &self.body;

        /// If we're substituting the bound variable ignore.
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



/// Implementation of [`PartialEq`] for [`Abstraction`]. States that two [`Abstraction`]s
/// are equal if they share the same bound variable and the same body expression. Any
/// non-abstraction is automatically considered unequal.
impl PartialEq<LambdaEntity> for Abstraction {
    fn eq(&self, other: &LambdaEntity) -> bool {
        match other {
            LambdaEntity::Abs(other_abs) => self.bound_var == other_abs.bound_var && self.body == other_abs.body,
            _ => return false
        }

    }
}



/// Implementation of '_pretty print_' for [`Abstraction`]. Used to generate
/// displays of larger λ-calculus expressions, follows `agda` syntax.
impl fmt::Display for Abstraction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "(λ{} → {})", self.bound_var, self.body)
    }
}

