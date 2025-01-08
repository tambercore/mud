use crate::lambda::abstraction::Abstraction;
use crate::lambda::application::Application;
use crate::lambda::variable::Variable;
use std::cmp::PartialEq;
use std::fmt;
use std::fmt::Formatter;

/// Substitutable trait (required by all elements of the LambdaEntity, or it will not work.)
pub trait Substitutable {
    fn substitute(&self, source: &LambdaEntity, target: &LambdaEntity) -> Box<Self>;
}

#[derive(Clone, Debug, PartialEq)]
pub enum LambdaEntity {
    App(Application),      // Application of two expressions
    Abs(Abstraction),      // Lambda abstraction, e.g., λx.x + 1
    Var(Variable),         // Variable, e.g., x
}


// Map to individual methods for pretty print.
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


// Map to individual methods for substitution.
impl Substitutable for LambdaEntity {
    fn substitute(&self, source: &LambdaEntity, target: &LambdaEntity) -> Box<Self> {
        match self {
            LambdaEntity::App(app) => { return app.substitute(source, target) }
            LambdaEntity::Abs(abs) => { return abs.substitute(source, target) }
            LambdaEntity::Var(var) => { return var.substitute(source, target) }
        }
    }
}