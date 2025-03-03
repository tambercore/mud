use std::cmp::PartialEq;
use crate::lambda::types::{Expandable, LambdaEntity, Substitutable};
use crate::lambda::conjunction::Conjunction;
use crate::{λCaseF, λConj, λPred};
use std::fmt;
use std::fmt::Formatter;
use crate::lambda::reducible::Reducible;

/// Structure to define Case Handlers i.e. for `is`
#[derive(Clone, Debug, PartialEq)]
pub struct CaseHandler {
    pub casef: Box<LambdaEntity>,
    pub casev: Box<LambdaEntity>
}


/// Implementation of Partial Equality for Case Handler, used in substitution.
impl PartialEq<LambdaEntity> for CaseHandler {
    fn eq(&self, other: &LambdaEntity) -> bool {
        if let LambdaEntity::CaseH(case) = other {
            self.casef == case.casef && self.casev == case.casev
        } else {
            false
        }
    }
}


/// Implementation of substitution for Case Handler.
impl Substitutable for CaseHandler {
    fn substitute(&self, source: &LambdaEntity, target: &LambdaEntity) -> Box<LambdaEntity> {

        // Am I being substituted? If so, replace me!
        let self_as_entity = LambdaEntity::CaseH(self.clone());
        if source == &self_as_entity {
            return Box::new(target.clone());
        }

        // Otherwise, substitute within my arguments
        let casef_s = self.casef.substitute(source, target);
        let casev_s = self.casev.substitute(source, target);


        λCaseF!(casef_s, casev_s)
    }
}



/// Implementation of Pretty Prints for CaseHandlers
impl fmt::Display for CaseHandler {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "λᶜ f[{}] or v[{}]", self.casef, self.casev)
    }
}