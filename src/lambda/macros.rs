
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