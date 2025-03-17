use crate::composer::structures::AgdaType;



/// Structure to define lambda-calculus in terms of an Agda AST. Terms in the LC
/// can be either a `Term`, `Abstraction` or `Application`. Beta-reduction here
/// is not implemented as it is handled inside of `Agda`.
#[derive(Clone, Debug, PartialEq)]
pub enum AgdaAst {
    Term(String),
    LambdaAbstraction {
        param: String,
        body: Box<AgdaAst>,
    },
    LambdaApplication {
        func: Box<AgdaAst>,
        arg: Box<AgdaAst>,
    },
}



/// Macro to construct a λ-term
#[macro_export]
macro_rules! astTerm {
    ($s:expr) => {
        Box::new(AgdaAst::Term($s))
    };
}



/// Macro to construt a λ-Abs
#[macro_export]
macro_rules! astLambda {
    ($param:expr, $body:expr) => {
        Box::new(AgdaAst::LambdaAbstraction {
            param: $param.to_string(),
            body: $body,
        })
    };
}



/// Macro to consturct a λ-App
#[macro_export]
macro_rules! astApply {
    ($func:expr, $arg:expr) => {
        Box::new(AgdaAst::LambdaApplication {
            func: $func,
            arg: $arg,
        })
    };
}