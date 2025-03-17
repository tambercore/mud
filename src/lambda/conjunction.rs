use std::cmp::PartialEq;
use crate::lambda::types::{Expandable, LambdaEntity, Substitutable};
use crate::{λConj, λPred, λVar};
use std::fmt;
use std::fmt::Formatter;
use crate::lambda::variable::Variable;



/// Structure to define Conjunctions e.g. a ^ b
#[derive(Clone, Debug, PartialEq)]
pub struct Conjunction {
    pub lhs: Box<LambdaEntity>,
    pub rhs: Box<LambdaEntity>
}


/// Implementation of Partial Equality for Conjunctions, used in substitution.
impl PartialEq<LambdaEntity> for Conjunction {
    fn eq(&self, other: &LambdaEntity) -> bool {
        match other {
            LambdaEntity::Conj(other_name) => *self.lhs == *other_name.lhs && *self.rhs == *other_name.rhs,
            _ => false
        }
    }
}


/// Implementation of substitution for Conjunctions.
impl Substitutable for Conjunction {
    fn substitute(&self, source: &LambdaEntity, target: &LambdaEntity) -> Box<LambdaEntity> {
        // Am I being substituted? If so, replace me!
        let self_as_entity = LambdaEntity::Conj(self.clone());
        if source == &self_as_entity {
            return Box::new(target.clone());
        }

        // Otherwise, substitute within Conjunction arguments
        λConj!(self.lhs.substitute(source, target).clone(), self.rhs.substitute(source, target).clone())
    }
}


/// Implementation of Pretty Prints for Conjunctions
impl fmt::Display for Conjunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} × {}", self.lhs, self.rhs)

    }
}


// Expand both sides, then return.
impl Expandable for Conjunction {
    fn expand(&self) -> Box<LambdaEntity> {
        λConj!(
            self.lhs.clone().expand(),
            self.rhs.clone().expand()
        )
    }
}