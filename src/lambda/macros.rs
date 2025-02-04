use crate::ccg::rule::CCGRule;
use crate::lambda::dependent_function::DependentFunction;
use crate::lambda::dependent_sum::DependentSum;

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

#[macro_export]
macro_rules! λPred {
    ($name:expr, $args:expr) => {
        Box::from(LambdaEntity::Pred(Predicate{iden: $name, args: $args}))
    };
}

#[macro_export]
macro_rules! λConj {
    ($lhs:expr, $rhs:expr) => {
        Box::from(LambdaEntity::Conj(Conjunction{lhs: $lhs, rhs: $rhs}))
    };
}

#[macro_export]
macro_rules! λDepFun {
    ($bound_var:expr, $expr:expr) => {
        Box::from(LambdaEntity::DepFun(DependentFunction{bound_var: $bound_var, expr: $expr}))
    };
}

#[macro_export]
macro_rules! λDepSum {
    ($bound_var:expr, $expr:expr) => {
        Box::from(LambdaEntity::DepSum(DependentSum{bound_var: $bound_var, expr: $expr}))
    };
}