/* AST definitions for Agda terms, abstractions, and applications */
use crate::composer::structures::AgdaType;

#[derive(Clone, Debug, PartialEq)]
pub enum AgdaAst {
    /// A simple variable or term identifier
    Term(String),

    /// Lambda abstraction (e.g., λ x → expr)
    Lambda {
        param: String,
        body: Box<AgdaAst>,
    },

    /// Function application (e.g., f x)
    Application {
        func: Box<AgdaAst>,
        arg: Box<AgdaAst>,
    },
}

/* Macro helpers for concise AST creation */

#[macro_export]
macro_rules! astTerm {
    ($s:expr) => {
        Box::new(AgdaAst::Term($s))
    };
}

#[macro_export]
macro_rules! astLambda {
    ($param:expr, $body:expr) => {
        Box::new(AgdaAst::Lambda {
            param: $param.to_string(),
            body: $body,
        })
    };
}

#[macro_export]
macro_rules! astApply {
    ($func:expr, $arg:expr) => {
        Box::new(AgdaAst::Application {
            func: $func,
            arg: $arg,
        })
    };
}