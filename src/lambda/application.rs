use crate::lambda::types::{LambdaEntity, Substitutable};
use crate::λApp;
use std::fmt;
use std::fmt::Formatter;



/// Structure to define [`Application`]/λ-App. These represent an application
/// in the λ-calculus (λx. e @ t), where one expression is applied to another.
#[derive(Clone, Debug, PartialEq)]
pub struct Application {
    pub lhs: Box<LambdaEntity>,
    pub rhs: Box<LambdaEntity>,
}



/// Implementation of [`Substitutable`] for [`Application`]. This captures the
/// process of substituting the source λ-term with the target λ-term in both
/// the left-hand side and right-hand side expressions.
impl Substitutable for Application {
    fn substitute(&self, source: &LambdaEntity, target: &LambdaEntity) -> Box<LambdaEntity> {
        let left = &self.lhs;
        let right = &self.rhs;

        let left_substituted = left.substitute(source, target);
        let right_substituted = right.substitute(source, target);
        λApp!(left_substituted, right_substituted)
    }
}



/// Implementation of `PartialEq` for [`Application`]. States that two [`Application`]s
/// are equal if their left-hand side and right-hand side are pairwise equal. Any
/// non-application entity is considered unequal.
impl PartialEq<LambdaEntity> for Application {
    fn eq(&self, other: &LambdaEntity) -> bool {
        match other {
            LambdaEntity::App(other_app) => self.lhs == other_app.lhs && self.rhs == other_app.rhs,
            _ => false
        }
    }
}



/// Implementation of '_pretty print_' for [`Application`]. Used
/// to generate displays of larger λ-calculus expressions. This
/// uses positional syntax, but can be altered to use `@` syntax.
impl fmt::Display for Application {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // It's also possible to include a explicit application operator i.e. ( term @ term ).
        write!(f, "({} {})", self.lhs, self.rhs)
    }
}