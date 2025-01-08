use super::lambda_element::LambdaElement;
use crate::lambda::abstraction::Abstraction;
use crate::lambda::application::Application;
use crate::lambda::variable::Variable;
use std::cmp::PartialEq;
use std::fmt;
use std::fmt::Formatter;

pub trait Substitutable {
    fn substitute(&self, source: &LambdaEntity, target: &LambdaEntity) -> Box<Self>;
}


#[derive(Clone, Debug, PartialEq)]
pub enum LambdaEntity {
    App(Application),      // Application of two expressions
    Abs(Abstraction),      // Lambda abstraction, e.g., λx.x + 1
    Var(Variable),         // Variable, e.g., x
}


impl fmt::Display for LambdaEntity {

    // Tedious, but necessary unless we want another library.
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            LambdaEntity::App(internal) => write!(f, "{}", internal),
            LambdaEntity::Abs(internal) => write!(f, "{}", internal),
            LambdaEntity::Var(internal) => write!(f, "{}", internal)
        }
    }
}