use crate::ast::agda_expr::AgdaExpr;

/// A type to denote Applications in Agda.
/// These are of the form a b, with a and b being Agda Expressions.
pub struct TApplication {pub lhs: Box<AgdaExpr>, pub rhs: Box<AgdaExpr>}

impl PartialEq for TApplication {
    fn eq(&self, other: &Self) -> bool {
        self.lhs == other.lhs && self.rhs == other.rhs
    }
}

#[macro_export]
macro_rules! app {
    ($lhs:expr, $rhs:expr) => {
        TApplication {
            lhs: Box::new($lhs),
            rhs: Box::new($rhs),
        }
    };
}