use crate::ast::agda_expr::AgdaExpr;

/// A type to denote record projection in Agda.
/// These are of the form a.b, with a and b being Agda Expressions.
#[derive(Clone)]
pub struct RecordProjection {pub lhs : Box<AgdaExpr>, pub rhs : Box<AgdaExpr>}

impl PartialEq for RecordProjection {
    fn eq(&self, other: &Self) -> bool {
        self.lhs == other.lhs && self.rhs == other.rhs
    }
}
#[macro_export]
macro_rules! record_projection {
    ($lhs:expr, $rhs:expr) => {
        RecordProjection {
            lhs: Box::new($lhs),
            rhs: Box::new($rhs),
        }
    };
}