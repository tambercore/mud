use std::cmp::PartialEq;
use crate::lambda::types::{LambdaEntity, Substitutable};
use crate::λPred;
use std::fmt;
use std::fmt::Formatter;

/// Structure to define λ-Applications (λx. e @ t)
#[derive(Clone, Debug, PartialEq)]
pub struct Predicate {
    pub iden: String,
    pub args: Vec<Box<LambdaEntity>>
}


/// Implementation of Partial Equality for Predicates, used in substitution.
impl PartialEq<LambdaEntity> for Predicate {
    fn eq(&self, other: &LambdaEntity) -> bool {
        if let LambdaEntity::Pred(pred) = other {
            self.iden == pred.iden && self.args == pred.args
        } else {
            false
        }
    }
}


/// Implementation of substitution for Predicates.
impl Substitutable for Predicate {
    fn substitute(&self, source: &LambdaEntity, target: &LambdaEntity) -> Box<LambdaEntity> {
        // Am I being substituted? If so, replace me!
        let self_as_entity = LambdaEntity::Pred(self.clone());
        if source == &self_as_entity {
            return Box::new(target.clone());
        }

        // Otherwise, substitute within predicate arguments
        let substituted_args: Vec<Box<LambdaEntity>> = self.args
            .iter()
            .map(|arg| arg.substitute(source, target))
            .collect();

        λPred!(self.name.clone(), substituted_args);
    }
}


/// Implementation of Pretty Prints for Predicates
impl fmt::Display for Predicate {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // It's also possible to include a explicit application operator i.e. ( term @ term ).
        if self.args.is_empty() { write!(f, "{}", self.iden) }
        else {
            let args_str = self.args.iter().map(|arg| arg.to_string()).collect::<Vec<String>>().join(", ");
            write!(f, "{}({})", self.iden, args_str)
        }
    }
}

