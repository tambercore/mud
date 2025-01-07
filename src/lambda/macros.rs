use super::types::*;

#[macro_export]
macro_rules! λVar {
    ($type_expr:expr) => {
        Box::from(LambdaEntity::Variable(Box::from($type_expr)))
    };
}

#[macro_export]
macro_rules! λAbs {
    ($left:expr, $right:expr) => {
        Box::from(LambdaEntity::Abstraction($left, $right))
    };
}

#[macro_export]
macro_rules! λApp {
    ($left:expr, $right:expr) => {
        Box::from(LambdaEntity::Application($left, $right))
    };
}