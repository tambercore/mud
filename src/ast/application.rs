use crate::ast::agda_expr::AgdaExpr;

/// A type to denote Applications in Agda.
/// These are of the form a b, with a and b being Agda Expressions.
#[derive(Clone)]
pub struct Application {pub lhs: Box<AgdaExpr>, pub rhs: Box<AgdaExpr>}

impl PartialEq for Application {
    fn eq(&self, other: &Self) -> bool {
        self.lhs == other.lhs && self.rhs == other.rhs
    }
}

#[macro_export]
macro_rules! app {
    ($lhs:expr, $rhs:expr) => {
        AgdaExpr::App (Application {
            lhs: Box::new($lhs),
            rhs: Box::new($rhs),
        })
    };
}