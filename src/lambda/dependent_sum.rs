use std::cmp::PartialEq;
use crate::lambda::types::{Expandable, LambdaEntity, Substitutable};
use crate::lambda::conjunction::Conjunction;
use crate::{λConj, λPred, λDepSum};
use std::fmt;
use std::fmt::Formatter;
use crate::lambda::reducible::Reducible;

/// Structure to define Σ(x) (expr)
#[derive(Clone, Debug, PartialEq)]
pub struct DependentSum {
    pub bound_var: Box<LambdaEntity>,
    pub expr: Box<LambdaEntity>,
}


/// Implementation of Partial Equality for DependentSum, used in substitution.
impl PartialEq<LambdaEntity> for DependentSum {
    fn eq(&self, other: &LambdaEntity) -> bool {
        if let LambdaEntity::DepSum(fun) = other {
            self.bound_var == fun.bound_var && self.expr == fun.expr
        } else {
            false
        }
    }
}


/// Implementation of substitution for DependentSum.
impl Substitutable for DependentSum {
    fn substitute(&self, source: &LambdaEntity, target: &LambdaEntity) -> Box<LambdaEntity> {
        // Am I being substituted? If so, replace me!
        let self_as_entity = LambdaEntity::DepSum(self.clone());
        if source == &self_as_entity {
            return Box::new(target.clone());
        }

        λDepSum!(self.bound_var.substitute(source, target), self.expr.substitute(source, target))
    }
}


impl Reducible for DependentSum {
    fn beta_reduce(&self) -> LambdaEntity {
        *λDepSum!(Box::from(self.bound_var.beta_reduce()), Box::from(self.expr.beta_reduce()))
    }
}


/// Implementation of Pretty Prints for DependentSum
impl fmt::Display for DependentSum {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Σ({}) ({})", self.bound_var, self.expr)
    }
}



/// Implements the `Expandable` trait for `DependentSum`, enabling recursive expansion logic.
impl Expandable for DependentSum {
    fn expand(&self) -> Box<LambdaEntity> {
        match &*self.bound_var {
            LambdaEntity::Conj(conjunction) => {
                let lhs_expanded = λDepSum!(conjunction.lhs.clone(), self.expr.substitute(&self.bound_var, &conjunction.lhs)).expand();
                let rhs_expanded = λDepSum!(conjunction.rhs.clone(), self.expr.substitute(&self.bound_var, &conjunction.rhs)).expand();

                Box::from(λConj!(lhs_expanded, rhs_expanded))
            }
            _ => {
                Box::from(λDepSum!(self.bound_var.expand(), self.expr.expand()))
            }
        }
    }
}
