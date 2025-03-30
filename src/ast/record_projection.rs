use crate::ast::agda_expr::AgdaExpr;

/// A type to denote record projection in Agda.
/// These are of the form a.b, with a and b being Agda Expressions.
#[derive(Eq, Hash, Debug, Clone)]
pub struct RecordProjection {pub lhs : String, pub rhs : Box<AgdaExpr>}

impl PartialEq for RecordProjection {
    fn eq(&self, other: &Self) -> bool {
        self.lhs == other.lhs && self.rhs == other.rhs
    }
}
#[macro_export]
macro_rules! record_projection {
    ($lhs:expr, $rhs:expr) => {
        AgdaExpr::RecProj(
        RecordProjection {
            lhs: String::from($lhs),
            rhs: Box::from($rhs),
        })
    };
}