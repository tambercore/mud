use crate::lambda::types::{LambdaEntity, Substitutable};
use crate::λApp;


/// Structure to define λ-Applications (λx. e @ t)
pub struct Application {
    pub term_one: Box<LambdaEntity>,
    pub term_two: Box<LambdaEntity>,
}


/// Implementation of λ-substitution for λ-Applications.
impl Substitutable for Application {
    fn substitute(&self, source: &LambdaEntity, target: &LambdaEntity) -> Box<LambdaEntity> {
        let left = &self.term_one;
        let right = &self.term_two;

        let left_substituted = left.substitute(source, target);
        let right_substituted = right.substitute(source, target);
        λApp!(left_substituted, right_substituted)
    }
}


/// Implementation of Partial Equality for λ-Applications, used in substitution.
impl PartialEq for Application {
    fn eq(&self, other: &Self) -> bool {
        self.term_one == other.term_one && self.term_two == other.term_two
    }
}
