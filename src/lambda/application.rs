use crate::lambda::types::{LambdaEntity, Substitutable};
use crate::λApp;
use std::fmt;
use std::fmt::Formatter;

/// Structure to define λ-Applications (λx. e @ t)
#[derive(Clone, Debug, PartialEq)]
pub struct Application {
    pub lhs: Box<LambdaEntity>,
    pub rhs: Box<LambdaEntity>,
}


/// Implementation of λ-substitution for λ-Applications.
impl Substitutable for Application {
    fn substitute(&self, source: &LambdaEntity, target: &LambdaEntity) -> Box<LambdaEntity> {
        let left = &self.lhs;
        let right = &self.rhs;

        let left_substituted = left.substitute(source, target);
        let right_substituted = right.substitute(source, target);
        λApp!(left_substituted, right_substituted)
    }
}


/// Implementation of Partial Equality for λ-Applications, used in substitution.
impl PartialEq<LambdaEntity> for Application {
    fn eq(&self, other: &LambdaEntity) -> bool {
        match other {
            LambdaEntity::App(other_app) => self.lhs == other_app.lhs && self.rhs == other_app.rhs,
            _ => false
        }
    }
}


/// Implementation of Pretty Prints for Applications
impl fmt::Display for Application {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // It's also possible to include a explicit application operator i.e. ( term @ term ).
        write!(f, "({} {})", self.lhs, self.rhs)
    }
}

