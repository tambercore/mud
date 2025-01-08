use crate::lambda::types::LambdaEntity::Var;
use super::types::*;
use super::abstraction::Abstraction;
use super::application::Application;
use super::variable::Variable;

#[macro_export]
macro_rules! λVar {
    ($type_expr:expr) => {
        Box::from(LambdaEntity::Var(Variable{name: $type_expr}))
    };
}

#[macro_export]
macro_rules! λAbs {
    ($left:expr, $right:expr) => {
        Box::from(LambdaEntity::Abs(Abstraction{bound_var: $left, body: $right}))
    };
}

#[macro_export]
macro_rules! λApp {
    ($left:expr, $right:expr) => {
        Box::from(LambdaEntity::App(Application{lhs: $left, rhs: $right}))
    };
}