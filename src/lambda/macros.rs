use crate::ccg::rule::CCGRule;
use crate::lambda::casef::CaseHandler;
use crate::lambda::types::LambdaEntity;

#[macro_export]
macro_rules! λVar {
    ($name_expr:expr) => {
        Box::from(LambdaEntity::Var(Variable{name: $name_expr, id: None}))
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
macro_rules! λCaseF {
    ($fcase:expr, $vcase:expr) => {
        Box::from(LambdaEntity::CaseH(CaseHandler{casef: $fcase, casev: $vcase}))
    };
}