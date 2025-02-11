use std::cmp::PartialEq;
use crate::lambda::types::{Expandable, LambdaEntity, Substitutable};
use crate::lambda::conjunction::Conjunction;
use crate::{λConj, λPred, λDepFun};
use std::fmt;
use std::fmt::Formatter;
use crate::lambda::reducible::Reducible;
use crate::lambda::types::LambdaEntity::Var;
use crate::lambda::variable::Variable;

/// Structure to define Π(x) (expr)
#[derive(Clone, Debug, PartialEq)]
pub struct DependentFunction {
    pub bound_var: Box<LambdaEntity>,
    pub expr: Box<LambdaEntity>,
}


/// Implementation of Partial Equality for DependentFunction, used in substitution.
impl PartialEq<LambdaEntity> for DependentFunction {
    fn eq(&self, other: &LambdaEntity) -> bool {
        if let LambdaEntity::DepSum(fun) = other {
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

        // If the EXPR of a dependent function is an abstraction, perform substitution inside the EXPR.
        match *self.clone().expr {
            LambdaEntity:: Abs(abstraction) => {
                λDepFun!(self.bound_var.substitute(&*abstraction.clone().bound_var, target), self.expr.substitute(&*abstraction.clone().bound_var, target))
            }
            _ => {λDepFun!(self.clone().bound_var, self.clone().expr)}
        }
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
        if let (Var(variable)) = *self.clone().bound_var {
            write!(f, "Π({}: {}) ({})", variable.name, variable.var_type, self.expr)
        }
        else {
            write!(f, "Π({}) ({})", self.bound_var, self.expr)
        }

        }
    }



/// Implements the `Expandable` trait for `DependentFunction`, enabling recursive expansion logic.
impl Expandable for DependentFunction {
    fn expand(&self) -> Box<LambdaEntity> {
        match &*self.bound_var {
            LambdaEntity::Conj(conjunction) => {
                let lhs_expanded = λDepFun!(conjunction.lhs.clone(), self.expr.substitute(&self.bound_var, &conjunction.lhs)).expand();
                let rhs_expanded = λDepFun!(conjunction.rhs.clone(), self.expr.substitute(&self.bound_var, &conjunction.rhs)).expand();

                Box::from(λConj!(lhs_expanded, rhs_expanded))
            }
            _ => {
                Box::from(λDepFun!(self.bound_var.expand(), self.expr.expand()))
            }
        }
    }
}
